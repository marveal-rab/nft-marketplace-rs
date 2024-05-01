use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::Method;
use axum::Router;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use app_state::AppState;

mod app_state;
mod domain;
mod errors;
mod middlewares;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_state: AppState = AppState::new();

    let app = Router::new()
        .merge(services::graphql_playground_router())
        .merge(services::graphql_router(app_state))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(middlewares::handle_error))
                .timeout(Duration::from_secs(30)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        );

    println!("Playground: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
