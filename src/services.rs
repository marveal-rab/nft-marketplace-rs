use std::sync::Arc;

use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{State, ws::WebSocketUpgrade},
    http::header::HeaderMap,
    middleware,
    response::{Html, IntoResponse, Response},
    Router,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    domain::models::token::{on_connection_init, Token},
    middlewares::auth,
    routers::api::upload_api,
};
use crate::errors::AppError;
use crate::routers::api::token_api::generate;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

fn get_token_from_headers(headers: &HeaderMap) -> Result<Token, AppError> {
    match headers
        .get("authorization")
        .map(|value| value.to_str().unwrap_or_default())
    {
        Some(access_token) => Token::parse_from_access_token(access_token.to_string()),
        None => {
            return Err(AppError::MissingCredentials);
        }
    }
}

async fn graphql_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, AppError> {
    let mut req = req.into_inner();
    match get_token_from_headers(&headers) {
        Ok(token) => {
            req = req.data(token);
            Ok(app_state.schema.execute(req).await.into())
        }
        Err(err) => Err(err),
    }
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

pub fn no_auth_api_router(app_state: Arc<AppState>) -> Router {
    let state: AppState = app_state.as_ref().clone();
    Router::new()
        .route("/api/v0/token/:addr", get(generate))
        .with_state(state)
}
