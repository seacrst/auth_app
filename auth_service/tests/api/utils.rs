use std::{str::FromStr, sync::Arc};

use auth_service::{get_postgres_pool, get_redis_client, services::{constants::{test, DATABASE_URL, DEFAULT_REDIS_HOSTNAME}, postgres_user_store::PostgresUserStore, postmark_email_client::PostmarkEmailClient, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore, two_fa::TwoFaCodeStore, MockEmailClient}, user::Email};
#[allow(dead_code, unused)]

use auth_service::{
    app::{state::AppState, App}, 
    services::tokens::{BannedTokenStoreError, BannedTokenStoreType, BannedTokens}, 
    user::store::Users
};
use reqwest::{cookie::Jar, Client, Response};
use secrecy::Secret;
use serde::Serialize;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, Connection, Executor, PgConnection, PgPool};
use tokio::sync::RwLock;
use uuid::Uuid;
use wiremock::MockServer;

pub struct TestApp {
    pub addr: String,
    pub client: Client,
    pub cookie_jar: Arc<Jar>,
    pub banned_tokens: BannedTokenStoreType,
    pub two_fa_code: Arc<RwLock<RedisTwoFACodeStore>>,
    pub db_name: String,
    pub email_server: MockServer, 
    pub clean_up_called: bool
}

impl TestApp {
    pub async fn new() -> Self {
        let email_server = MockServer::start().await;
        let base_url = email_server.uri(); // New!
        let email_client = Arc::new(configure_postmark_email_client(base_url));
        
        let redis_connection = Arc::new(RwLock::new(configure_redis()));
        let db_name = Uuid::new_v4().to_string();
        let pg_pool = configure_postgresql(&db_name).await;
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
            redis_connection.clone(),
        )));
        let two_fa_code = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));

        let app_state = AppState {
            banned_token_store: banned_token_store.clone(),
            two_fa_code: two_fa_code.clone(),
            email_client,
            user_store
        };

        let app = App::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let addr = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self { 
            addr, 
            client, 
            cookie_jar, 
            banned_tokens: banned_token_store,
            two_fa_code,
            db_name,
            email_server,
            clean_up_called: false
        }
    }

    pub async fn get_root(&self) -> Response {
        self.client
            .get(&format!("{}/", &self.addr))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Body: serde::Serialize>(&self, body: &Body) -> Response {
        self.client.post(&format!("{}/login", &self.addr))
        .json(body)
        .send().await
        .expect("Failed to execute request")
    }

    pub async fn get_signup(&self) -> Response {
        self.client
            .post(&format!("{}/signup", &self.addr))
            .send()
            .await
            .expect("Failed to execute request")
    }
    pub async fn get_login(&self) -> Response {
        self.client
            .post(&format!("{}/login", &self.addr))
            .send()
            .await
            .expect("Failed to execute request")
    }
    pub async fn get_logout(&self) -> Response {
        self.client
            .post(&format!("{}/logout", &self.addr))
            .send()
            .await
            .expect(" -> StringFailed to execute request")
    }
    pub async fn get_verify_2fa(&self) -> Response {
        self.client
            .post(&format!("{}/verify-2fa", &self.addr))
            .send()
            .await
            .expect("Failed to execute request")
    }
    pub async fn get_verify_token(&self) -> Response {
        self.client
            .post(&format!("{}/verify-token", &self.addr))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize {
        self.client
            .post(&format!("{}/signup", &self.addr))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.client
            .post(format!("{}/logout", &self.addr))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where Body: Serialize {
        self.client
            .post(format!("{}/verify-2fa", &self.addr))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize {
        self.client
            .post(format!("{}/verify-token", &self.addr))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&mut self) {
        if self.clean_up_called {
            return;
        }

        delete_database(&self.db_name).await;

        self.clean_up_called = true;
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("TestApp::clean_up was not called before dropping TestApp");
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql(db_name: &str) -> PgPool {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    // We are creating a new database for each test case, and we need to ensure each database has a unique name!

    configure_database(&postgresql_conn_url, &db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    // Create a new connection pool and return it
    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}


async fn configure_database(db_conn_string: &str, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");


    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}

async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}


fn configure_redis() -> redis::Connection {
    let redis_hostname = DEFAULT_REDIS_HOSTNAME.to_owned();

    get_redis_client(redis_hostname)
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

fn configure_postmark_email_client(base_url: String) -> PostmarkEmailClient {
    let postmark_auth_token = Secret::new("auth_token".to_owned());

    let sender = Email::parse(test::email_client::SENDER.to_owned()).unwrap();

    let http_client = Client::builder()
        .timeout(test::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(base_url, sender, postmark_auth_token, http_client)
}