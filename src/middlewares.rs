use axum::http::{Method, StatusCode, Uri};
use axum::BoxError;

pub async fn handle_error(method: Method, uri: Uri, err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("`{method} {uri}` failed with {err}"),
        )
    }
}
