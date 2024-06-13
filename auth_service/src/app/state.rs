
use crate::{services::tokens::BannedTokenStoreType, user::store::UserStoreType};

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokenStoreType) -> Self {
        Self { user_store, banned_token_store }
    }
}