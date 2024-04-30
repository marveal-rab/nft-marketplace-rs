use axum::{
    extract::Request,
    http::{self},
    middleware::Next,
    response::Response,
};

use crate::domain::models::token::Token;
use crate::errors::AppError;

#[derive(Clone)]
pub struct CurrentUser {
    pub address: String,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(AppError::MissingCredentials);
    };

    return match authorize_current_user(auth_header).await {
        Ok(current_user) => {
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        Err(err) => Err(err),
    };
}

async fn authorize_current_user(auth_token: &str) -> Result<CurrentUser, AppError> {
    return match Token::parse_from_access_token(auth_token.to_string()) {
        Ok(token) => {
            return match token.parse().map(|encrypt_user_info| CurrentUser {
                address: encrypt_user_info.address,
            }) {
                Ok(current_user) => Ok(current_user),
                Err(err) => Err(err),
            };
        }
        Err(err) => Err(err),
    };
}
