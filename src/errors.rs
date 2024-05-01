use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, PartialEq)]
pub enum AppError {
    // TOKEN
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,

    // UPLOAD
    UploadMissingFile,

    // IPFS
    RequestIpfsAddFailed,
    RequestIpfsAddError,
    RequestIpfsAddResponseNoBody,
    RequestIpfsAddResponseBodyDeserializeFailed,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AppError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AppError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AppError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AppError::RequestIpfsAddFailed
            | AppError::RequestIpfsAddError
            | AppError::RequestIpfsAddResponseNoBody
            | AppError::RequestIpfsAddResponseBodyDeserializeFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to upload file to IPFS",
            ),
            AppError::UploadMissingFile => (StatusCode::BAD_REQUEST, "missing file"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown Error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AppError {}
