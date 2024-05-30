use std::sync::Arc;
use auth_service::{
    app_state::AppState, 
    services::UserDataStore, App
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {

    let app_state = AppState::new(Arc::new(RwLock::new(UserDataStore::default())));
    
    let app = App::build(app_state, "0.0.0.0:4000").await.expect("Failed to build app");
    app.run().await.expect("Failed to run app")
}
