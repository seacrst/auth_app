
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{services::two_fa::TwoFaCodes, services::tokens::BannedTokenStoreType, user::store::UserStoreType};

use super::email_client::EmailClient;

pub type TwoFaCodeStoreType = Arc<RwLock<dyn TwoFaCodes + Send + Sync>>;
pub type EmailClientType = Arc<RwLock<dyn EmailClient + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code: TwoFaCodeStoreType,
    pub email_client: EmailClientType
}