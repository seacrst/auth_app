use std::sync::Arc;

use auth_service::{app_state::AppState, services::UserDataStore, App};
use reqwest::Client;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
  pub addr: String,
  pub client: Client
}

impl TestApp {
  pub async fn new() -> Self {
    let user_store = UserDataStore::default();
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

    let app = App::build(app_state, "127.0.0.1:0").await.expect("Failed to build app");

    let addr = format!("http://{}", app.address.clone());

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(app.run());

    let client = Client::new();

    Self {addr, client}
  }

  pub async fn get_root(&self) -> reqwest::Response {
      self.client
          .get(&format!("{}/", &self.addr))
          .send()
          .await
          .expect("Failed to execute request")
  }

  pub async fn get_signup(&self) -> reqwest::Response {
      self.client
          .post(&format!("{}/signup", &self.addr))
          .send()
          .await
          .expect("Failed to execute request")
  }
  pub async fn get_login(&self) -> reqwest::Response {
      self.client
          .post(&format!("{}/login", &self.addr))
          .send()
          .await
          .expect("Failed to execute request")
  }
  pub async fn get_logout(&self) -> reqwest::Response {
      self.client
          .post(&format!("{}/logout", &self.addr))
          .send()
          .await
          .expect(" -> StringFailed to execute request")
  }
  pub async fn get_verify_2fa(&self) -> reqwest::Response {
      self.client
          .post(&format!("{}/verify-2fa", &self.addr))
          .send()
          .await
          .expect("Failed to execute request")
  }
  pub async fn get_verify_token(&self) -> reqwest::Response {
      self.client
          .post(&format!("{}/verify-token", &self.addr))
          .send()
          .await
          .expect("Failed to execute request")
  }

  pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize {
        self.client
            .post(&format!("{}/signup", &self.addr))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
  format!("{}@example.com", Uuid::new_v4())
}