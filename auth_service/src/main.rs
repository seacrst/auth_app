use std::sync::Arc;
use auth_service::{
    app::{state::AppState, App}, 
    services::{
        constants::prod, 
        tokens::BannedTokens,
        two_fa::{TwoFaCode, TwoFaCodeStore, TwoFaCodes}
    }, 
    user::store::Users
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(Users::default()));
    let banned_token_store = Arc::new(RwLock::new(BannedTokens::default()));
    let two_fa_code = Arc::new(RwLock::new(TwoFaCodeStore::default()));
    let app_state = AppState::new(store, banned_token_store, two_fa_code);
    
    let app = App::build(app_state, prod::APP_ADDRESS).await.expect("Failed to build app");
    app.run().await.expect("Failed to run app")
}
