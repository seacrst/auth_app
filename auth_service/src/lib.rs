use std::error::Error;

use axum::{routing::post, serve::Serve, Router};
use tower_http::services::ServeDir;

pub mod routes;
use routes::{login, logout, signup, verify_2fa, verify_token};
pub struct App {
    server: Serve<Router, Router>,
    pub address: String
}

impl App {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
      let router = Router::new()
      .nest_service("/", ServeDir::new("assets"))
      .route("/signup", post(signup))
      .route("/login", post(login))
      .route("/logout", post(logout))
      .route("/verify-2fa", post(verify_2fa))
      .route("/verify-token", post(verify_token));

      let listener = tokio::net::TcpListener::bind(address).await?;
      
      let address = listener.local_addr()?.to_string();
      let server = axum::serve(listener, router);


      Ok(App {address, server})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
      println!("listening on {}", &self.address);
      self.server.await
    }
}