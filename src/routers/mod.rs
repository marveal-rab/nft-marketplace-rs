use crate::errors::AppError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod api;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(success: bool, message: String, data: Option<T>) -> Self {
        return Self {
            success,
            message,
            data,
        };
    }

    pub fn success() -> Self {
        return Self::new(true, "success".to_string(), None);
    }

    pub fn success_with_data(data: T) -> Self {
        return Self::new(true, "success".to_string(), Some(data));
    }

    pub fn success_with_message_data(message: String, data: T) -> Self {
        return Self::new(true, message, Some(data));
    }

    pub fn failed() -> Self {
        return Self::new(false, "failed".to_string(), None);
    }

    pub fn failed_with_message(message: String) -> Self {
        return Self::new(false, message, None);
    }

    pub fn error(error: AppError) -> Self {
        return Self::new(false, error.to_string(), None);
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let (status, body) = match self.success {
            true => (StatusCode::OK, Json(json!(self))),
            false => (StatusCode::BAD_REQUEST, Json(json!(self))),
        };
        (status, body).into_response()
    }
}
