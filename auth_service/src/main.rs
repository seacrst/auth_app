use std::sync::Arc;
use auth_service::{
    app::{state::AppState, App}, get_postgres_pool, services::{
        constants::{prod, DATABASE_URL}, postgres_user_store::PostgresUserStore, tokens::BannedTokens, two_fa::TwoFaCodeStore, MockEmailClient
    }
};
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));

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
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}