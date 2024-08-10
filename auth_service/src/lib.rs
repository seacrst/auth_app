use redis::{Client, RedisResult};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod route_handlers;
pub mod app;
pub mod services;
pub mod user;

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}

