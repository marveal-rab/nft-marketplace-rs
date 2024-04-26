use std::sync::Arc;

use crate::{
    app_state::AppState,
    domain::models::token::{on_connection_init, Token},
    middlewares::auth,
    routers::api::upload_api,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::header::HeaderMap,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn graphql_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }
    app_state.schema.execute(req).await.into()
}

async fn graphql_websocket_handler(
    State(app_state): State<AppState>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, app_state.schema.clone(), protocol)
                .on_connection_init(on_connection_init)
                .serve()
        })
}

pub fn graphql_playground_router() -> Router {
    Router::new().route("/playground", get(graphql_playground))
}

pub fn graphql_router(app_state: Arc<AppState>) -> Router {
    let state: AppState = app_state.as_ref().clone();
    Router::new()
        .route("/", post(graphql_handler))
        .route("/ws", get(graphql_websocket_handler))
        .with_state(state)
}

pub fn api_router(app_state: Arc<AppState>) -> Router {
    let state: AppState = app_state.as_ref().clone();
    Router::new()
        .route("/api", post(graphql_handler))
        .route("/api/upload", post(upload_api::upload))
        .layer(middleware::from_fn(auth::auth))
        .with_state(state)
}
