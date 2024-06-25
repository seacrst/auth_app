use std::collections::HashMap;
use async_trait::async_trait;
use rand::Rng;
use uuid::Uuid;
use crate::{services::api::TwoFaCodeError, user::Email};


#[derive(Debug, Clone, PartialEq)]
pub struct LoginId(String);


impl LoginId {
    pub fn parse(id: String) -> Result<Self, String> {
        let valid_id = Uuid::parse_str(&id)
            .map_err(|_| String::from("Invalid Login ID format"))?;
        Ok(Self(valid_id.to_string()))
    }
}

impl Default for LoginId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        Self(id.to_string())
    }
}

impl AsRef<str> for LoginId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFaCode(String);

impl TwoFaCode {
    pub fn parse(code: String) -> Result<Self, String> {
        let err_invalid_format = String::from("Invalid code format");
        let digits = code.clone().parse::<u32>()
            .map_err(|_| err_invalid_format.clone())?;
        if (100_000..=999_999).contains(&digits) {
            Ok(Self(code))
        } else {
            Err(err_invalid_format)
        }
    }
}

impl Default for TwoFaCode {
    fn default() -> Self {
        let range = rand::thread_rng().gen_range(100_000..=999_999);
        Self(range.to_string())
    }
}

impl AsRef<str> for TwoFaCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Default)]
pub struct TwoFaCodeStore {
    codes: HashMap<Email, (LoginId, TwoFaCode)>,
}

#[async_trait]
pub trait TwoFaCodes {
    async fn add_code(&mut self, email: Email, login_id: LoginId, code: TwoFaCode) -> Result<(), TwoFaCodeError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFaCodeError>;
    async fn get_code(&self, email: &Email) -> Result<(LoginId, TwoFaCode), TwoFaCodeError>;
}

#[async_trait]
impl TwoFaCodes for TwoFaCodeStore {
    async fn add_code(&mut self, email: Email, login_id: LoginId, code: TwoFaCode) -> Result<(), TwoFaCodeError> {
        let _ = self.codes.insert(email, (login_id, code))
            .map(|_| ())
            .ok_or(TwoFaCodeError::UnexpectedError); // TODO
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFaCodeError> {
        self.codes.remove(email)
            .map(|_| ())
            .ok_or(TwoFaCodeError::UnexpectedError)
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginId, TwoFaCode), TwoFaCodeError> {
        self.codes.get(email)
            .map(|tup| tup.clone())
            .ok_or(TwoFaCodeError::LoginAttemptIdNotFound)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_add_code() {
        let mut store = TwoFaCodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_id = LoginId::default();
        let code = TwoFaCode::default();

        let result = store
            .add_code(email.clone(), login_id.clone(), code.clone())
            .await;

        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), Some(&(login_id, code)));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = TwoFaCodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginId::default();
        let code = TwoFaCode::default();

        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.remove_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), None);
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = TwoFaCodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginId::default();
        let code = TwoFaCode::default();
        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.get_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (login_attempt_id, code));
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = TwoFaCodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();

        let result = store.get_code(&email).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TwoFaCodeError::LoginAttemptIdNotFound
        );
    }
}