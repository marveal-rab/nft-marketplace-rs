use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::Router;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

use app_state::AppState;

mod app_state;
mod domain;
mod errors;
mod middlewares;
mod routers;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_state: Arc<AppState> = Arc::new(AppState::new());

    let app = Router::new()
        .merge(services::graphql_playground_router())
        .merge(services::graphql_router(app_state.clone()))
        .merge(services::api_router(app_state.clone()))
        .merge(services::no_auth_api_router(app_state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(middlewares::error::handle_error))
                .timeout(Duration::from_secs(30)),
        );

    println!("Playground: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
