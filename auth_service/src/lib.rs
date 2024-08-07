use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod api_handlers;
pub mod app;
pub mod services;
pub mod user;

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}