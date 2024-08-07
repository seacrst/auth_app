use std::error::Error;
use axum::{
    http::Method,
    routing::post,
    serve::Serve,
    Router
};
use config::AppConfig;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use crate::services::tracing::{make_span_with_request_id, on_request, on_response};

pub mod config;
pub mod state;
pub mod email_client;

use state::AppState;
use super::api_handlers::{
    login,
    logout,
    signup,
    verify_2fa,
    verify_token
};
pub struct App {
    server: Serve<Router, Router>,
    pub address: String,
}

impl App {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let AppConfig {paths} = AppConfig::config();
        
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://198.211.117.167:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route(paths.signup, post(signup))
            .route(paths.login, post(login))
            .route(paths.verify_2fa, post(verify_2fa))
            .route(paths.logout, post(logout))
            .route(paths.verify_token, post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(App { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}