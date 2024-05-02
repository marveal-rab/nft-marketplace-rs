use async_graphql::{Context, InputObject, Object, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::{errors::AppError, models::user::InsertedUser, models::user::User};

use super::token::Token;

#[derive(Default)]
pub struct UserMutation;
#[derive(Default)]
pub struct UserQuery;

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct NewUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CreateUserResult {
    pub id: String,
    pub name: Option<String>,
    pub address: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<User> for CreateUserResult {
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

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct UserResult {
    pub id: String,
    pub name: Option<String>,
    pub address: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl From<User> for UserResult {
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
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        user: NewUser,
    ) -> Result<CreateUserResult, AppError> {
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        let address = encrypt_user_info.address;
        let exists_user = User::find_by_address(address.clone()).await?;
        match exists_user {
            Some(user) => Ok(CreateUserResult::from(user)),
            None => InsertedUser {
                name: user.name,
                email: user.email,
                avatar_url: user.avatar_url,
                address: address.clone(),
            }
            .insert()
            .await
            .map(|user| CreateUserResult::from(user)),
        }
    }
}

#[Object]
impl UserQuery {
    pub async fn find_by_address(&self, ctx: &Context<'_>) -> Result<UserResult, AppError> {
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        let user = User::find_by_address(encrypt_user_info.address).await?;
        user.map(|user| UserResult::from(user))
            .ok_or(AppError::UserNotFound)
    }
}
