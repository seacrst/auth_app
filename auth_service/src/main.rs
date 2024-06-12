use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{
    app_state::AppState, 
    services::UserDataStore, 
    utils::constants::prod,
    App
};

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(UserDataStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(store, banned_token_store);
    
    let app = App::build(app_state, prod::APP_ADDRESS).await.expect("Failed to build app");
    app.run().await.expect("Failed to run app")
}
