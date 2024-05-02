use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

use super::{establish_connection, schema::users};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub address: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn find_by_address(address: String) -> Result<Option<User>, AppError> {
        let connection = &mut establish_connection();
        users::table
            .filter(users::address.eq(address))
            .first(connection)
            .map(|user| Some(user))
            .or_else(|err| {
                if err == diesel::NotFound {
                    Ok(None)
                } else {
                    Err(AppError::UserQueryError)
                }
            })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct InsertedUser {
    pub name: Option<String>,
    pub address: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl InsertedUser {
    pub async fn insert(&self) -> Result<User, AppError> {
        let connection = &mut establish_connection();
        return diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
            .map_err(|err| {
                tracing::error!("create user error: {:?}", err);
                AppError::CreateUserFailed
            });
    }
}
