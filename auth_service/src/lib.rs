pub mod domain;
pub mod services;
pub mod routes;
pub mod app_state;
pub mod utils;
pub mod app;

pub use routes::{login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};

use std::error::Error;

use axum::{
    response::{IntoResponse, Response},
    routing::post, 
    serve::Serve, 
    Router, 
    http,
    Json
};
use tower_http::{
    cors::CorsLayer, 
    services::ServeDir
};


use crate::domain::AuthApiError;
use crate::app_state::AppState;

use app::AppConfig;

pub struct App {
    server: Serve<Router, Router>,
    pub address: String,
}

impl App {
    pub async fn build(state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let AppConfig {paths} = AppConfig::config();

        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://[YOUR_DROPLET_IP]:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([http::Method::GET, http::Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);
        
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route(paths.signup, post(signup))
            .route(paths.login, post(login))
            .route(paths.logout, post(logout))
            .route(paths.verify_2fa, post(verify_2fa))
            .route(paths.verify_token, post(verify_token))
            .with_state(state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(App { address, server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthApiError::UserAlreadyExists => (http::StatusCode::CONFLICT, "User already exists"),
            AuthApiError::InvalidCredentials => (http::StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthApiError::MissingToken => (http::StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthApiError::InvalidToken => (http::StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthApiError::IncorrectCredentials => (http::StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthApiError::UnexpectedError => {
                (http::StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}