
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{services::two_fa::TwoFaCodes, services::tokens::BannedTokenStoreType, user::store::UserStoreType};

pub type TwoFaCodeStoreType = Arc<RwLock<dyn TwoFaCodes + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code: TwoFaCodeStoreType
}

impl AppState {
    pub fn new(
        user_store: UserStoreType, 
        banned_token_store: BannedTokenStoreType,
        two_fa_code: TwoFaCodeStoreType
    ) -> Self {
        Self { 
            user_store, 
            banned_token_store,
            two_fa_code
        }
    }
}