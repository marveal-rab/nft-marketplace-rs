use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http,
    http::header::HeaderMap,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};

use crate::errors::AppError;
use crate::{
    app_state::AppState,
    domain::token::{on_connection_init, Token},
};

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

async fn graphql_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, AppError> {
    let mut req = req.into_inner();
    tracing::info!("graphql_handler: {} {}", req.query, req.variables);

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
