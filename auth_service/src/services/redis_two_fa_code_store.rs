use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::user::Email;

use super::{api::TwoFaCodeError, two_fa::{LoginId, TwoFaCode, TwoFaCodes}};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFaCodes for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginId,
        code: TwoFaCode,
    ) -> Result<(), TwoFaCodeError> {
        let key = get_key(&email);

        let data = TwoFATuple(
            login_attempt_id.as_ref().to_owned(),
            code.as_ref().to_owned(),
        );
        let serialized_data =
            serde_json::to_string(&data).map_err(|_| TwoFaCodeError::UnexpectedError)?;

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(&key, serialized_data, TEN_MINUTES_IN_SECONDS)
            .map_err(|_| TwoFaCodeError::UnexpectedError)?;

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFaCodeError> {
        let key = get_key(email);

        let _: () = self
            .conn
            .write()
            .await
            .del(&key)
            .map_err(|_| TwoFaCodeError::UnexpectedError)?;

        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginId, TwoFaCode), TwoFaCodeError> {
        let key = get_key(email);

        match self.conn.write().await.get::<_, String>(&key) {
            Ok(value) => {
                let data: TwoFATuple = serde_json::from_str(&value)
                    .map_err(|_| TwoFaCodeError::UnexpectedError)?;

                let login_attempt_id = LoginId::parse(data.0)
                    .map_err(|_| TwoFaCodeError::UnexpectedError)?;

                let email_code =
                    TwoFaCode::parse(data.1).map_err(|_| TwoFaCodeError::UnexpectedError)?;

                Ok((login_attempt_id, email_code))
            }
            Err(_) => Err(TwoFaCodeError::LoginAttemptIdNotFound),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}