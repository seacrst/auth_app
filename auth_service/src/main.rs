use std::sync::Arc;
use auth_service::{
    app::{state::AppState, App}, get_postgres_pool, get_redis_client, services::{
        constants::{prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, REDIS_HOST_NAME}, postgres_user_store::PostgresUserStore, postmark_email_client::PostmarkEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore, tokens::BannedTokens, tracing::init_tracing, two_fa::TwoFaCodeStore, MockEmailClient
    }, user::Email
};
use reqwest::Client;
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    init_tracing();
    let email_client = Arc::new(configure_postmark_email_client()); 
    
    let pg_pool = configure_postgresql().await;
    let store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let two_fa_code = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    
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

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(prod::email_client::SENDER.to_owned()).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}