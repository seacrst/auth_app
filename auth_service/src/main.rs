use std::sync::Arc;
use auth_service::{
    app::{state::AppState, App}, 
    services::{
        constants::prod, 
        tokens::BannedTokens,
        two_fa::TwoFaCodeStore, MockEmailClient
    }, 
    user::store::Users
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(Users::default()));
    let banned_token_store = Arc::new(RwLock::new(BannedTokens::default()));
    let two_fa_code = Arc::new(RwLock::new(TwoFaCodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));
    
    let app_state = AppState {
        user_store: store,
        banned_token_store,
        two_fa_code,
        email_client
    };
    
    let app = App::build(app_state, prod::APP_ADDRESS).await.expect("Failed to build app");
    app.run().await.expect("Failed to run app")
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}