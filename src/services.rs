use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{State, ws::WebSocketUpgrade},
    http,
    http::header::HeaderMap,
    response::{Html, IntoResponse, Response},
    Router,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    domain::models::token::{on_connection_init, Token},
};
use crate::errors::AppError;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

async fn graphql_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, AppError> {
    let mut req = req.into_inner();
    tracing::info!("graphql_handler: {}", req.query);

    match headers
        .get(http::header::AUTHORIZATION)
        .map(|value| value.to_str().unwrap_or_default())
    {
        None => Ok(app_state.schema.execute(req).await.into()),
        Some(access_token) => match Token::parse_from_access_token(access_token.to_string()) {
            Ok(token) => {
                req = req.data(token);
                Ok(app_state.schema.execute(req).await.into())
            }
            Err(err) => Err(err),
        },
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

pub fn graphql_router(app_state: AppState) -> Router {
    Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/ws", get(graphql_websocket_handler))
        .with_state(app_state)
}
