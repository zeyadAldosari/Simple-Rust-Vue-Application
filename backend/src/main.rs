mod db;
mod handlers;
mod models;
mod schema;
mod utils;

use axum::{routing::{post}, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::http::{Method, HeaderValue};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5173"))
        .allow_methods([Method::POST])
        .allow_headers(Any);
          
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
    let app = Router::new()
        .route("/api/register", post(handlers::register_user))
        .route("/api/login", post(handlers::login_user))
        .layer(cors);  
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
