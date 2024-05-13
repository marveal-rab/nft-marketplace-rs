use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppError {
    SerializeUrlError,
    ParseUrlError,

    // TOKEN
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,

    // UPLOAD
    UploadMissingFile,
    HashMismatch,

    // IPFS
    RequestIpfsFailed,
    RequestIpfsError,
    RequestIpfsResponseNoBody,
    RequestIpfsResponseBodyDeserializeFailed,

    // DATABASE
    NoDatabaseConnection,

    // USER
    CreateUserFailed,
    UserNotFound,
    UserQueryError,
    // COLLECTION
    CollectionNotFound,
    CollectionQueryError,
    CreateCollectionFailed,
    // NFT
    NftNotFound,
    CreateNFTFailed,
    // NFT Trait
    NftTraitNotFound,
    CreateNFTTraitFailed,
    CountNFTFailed,

    NotImplemented,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AppError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AppError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AppError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AppError::RequestIpfsFailed
            | AppError::RequestIpfsError
            | AppError::RequestIpfsResponseNoBody
            | AppError::RequestIpfsResponseBodyDeserializeFailed => (
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
