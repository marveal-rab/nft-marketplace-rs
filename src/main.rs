use std::sync::Arc;

use app_state::AppState;

use axum::Router;

use tokio::net::TcpListener;

mod app_state;
mod domain;
mod middlewares;
mod routers;
mod services;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .merge(services::graphql_playground_router())
        .merge(services::graphql_router(app_state.clone()))
        .merge(services::api_router(app_state.clone()));

    println!("Playground: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
