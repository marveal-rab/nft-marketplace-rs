use axum::extract::Path;
use axum::Json;
use serde_json::{json, Value};

use crate::domain::models::token::Token;
use crate::errors::AppError;

pub async fn generate(Path(addr): Path<String>) -> Result<Json<Value>, AppError> {
    Token::generate(addr).map(|token| Json(json!(token)))
}
