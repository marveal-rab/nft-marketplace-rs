use std::env;
use std::string::ToString;

use async_graphql::{Context, Data, Object, SimpleObject, Subscription};
use chrono::{Duration, Utc};
use futures_util::Stream;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, models::user::User};

#[derive(Default)]
pub struct TokenMutation;
#[derive(Default)]
pub struct TokenQuery;
#[derive(Default)]
pub struct TokenSubscription;

#[derive(Serialize, Deserialize, Debug, Default, SimpleObject)]
pub struct Token {
    pub secret: String,
    pub token_type: String,
}

impl Token {
    fn new(secret: String) -> Self {
        Self {
            secret,
            token_type: "Bearer".to_string(),
        }
    }

    pub fn parse_from_access_token(access_token: String) -> Result<Self, AppError> {
        if access_token.starts_with("Bearer ") {
            Ok(Self::new(access_token.replacen("Bearer ", "", 1)))
        } else {
            Err(AppError::InvalidToken)
        }
    }

    pub fn generate(address: String) -> std::result::Result<Token, AppError> {
        let user_info = EncryptUserInfo::new(address);
        return encode(&Header::default(), &user_info, &KEYS.encoding)
            .map(|token| Token::new(token))
            .map_err(|_| AppError::TokenCreation);
    }

    pub fn parse(&self) -> std::result::Result<EncryptUserInfo, AppError> {
        return decode::<EncryptUserInfo>(
            self.secret.as_str(),
            &KEYS.decoding,
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|err| {
            tracing::error!("token parse error: {:?}", err);
            AppError::InvalidToken
        });
    }

    pub async fn current_user(&self) -> Result<User, AppError> {
        let encrypt_user_info = self.parse()?;
        User::find_by_address(encrypt_user_info.address)
            .await
            .map(|user| user.ok_or(AppError::UserNotFound))?
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptUserInfo {
    pub address: String,
    pub exp: usize,
}

impl EncryptUserInfo {
    pub fn new(address: String) -> Self {
        Self {
            address: address.to_lowercase(),
            exp: (Utc::now().naive_utc() + Duration::days(1))
                .and_utc()
                .timestamp() as usize,
        }
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CurrentUserResult {
    pub id: String,
    pub name: Option<String>,
    pub address: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<User> for CurrentUserResult {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            address: user.address,
            email: user.email,
            avatar_url: user.avatar_url,
        }
    }
}

#[Object]
impl TokenQuery {
    async fn current_user<'a>(&self, ctx: &Context<'_>) -> Result<CurrentUserResult, AppError> {
        let token = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?;
        token
            .current_user()
            .await
            .map(|user| CurrentUserResult::from(user))
    }
}

#[Subscription]
impl TokenSubscription {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data::<Token>()?.secret != "123456" {
            return Err("Forbidden".into());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}

#[Object]
impl TokenMutation {
    pub async fn generate_token(&self, address: String) -> Result<Token, AppError> {
        Token::generate(address)
    }
}

pub async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<Data> {
    #[derive(serde::Deserialize)]
    struct Payload {
        token: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        return match Token::parse_from_access_token(payload.token) {
            Ok(token) => {
                data.insert(token);
                Ok(data)
            }
            Err(err) => Err(async_graphql::Error::from(err)),
        };
    } else {
        Err("Invalid payload".into())
    }
}
