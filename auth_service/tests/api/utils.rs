use std::sync::Arc;

use auth_service::services::{two_fa::TwoFaCodeStore, MockEmailClient};
#[allow(dead_code, unused)]

use auth_service::{
    app::{state::AppState, App}, 
    services::tokens::{BannedTokenStoreError, BannedTokenStoreType, BannedTokens}, 
    user::store::Users
};
use reqwest::{cookie::Jar, Client, Response};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub addr: String,
    pub client: Client,
    pub cookie_jar: Arc<Jar>,
    pub banned_tokens: BannedTokenStoreType,
    pub two_fa_code: Arc<RwLock<TwoFaCodeStore>>,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(Users::default()));
        let banned_tokens = Arc::new(RwLock::new(BannedTokens::default()));
        let two_fa_code = Arc::new(RwLock::new(TwoFaCodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient));

        let app_state = AppState {
            banned_token_store: banned_tokens.clone(), 
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
            banned_tokens,
            two_fa_code
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

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.client
            .post(format!("{}/verify-2fa", &self.addr))
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
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
