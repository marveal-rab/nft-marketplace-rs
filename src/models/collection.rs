use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

use super::{establish_connection, schema::collections};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = collections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Collection {
    pub async fn find_by_owner(owner: String) -> Result<Vec<Collection>, AppError> {
        let connection = &mut establish_connection();
        collections::table
            .filter(collections::owner.eq(owner))
            .load(connection)
            .map_err(|err| {
                tracing::error!("find collection error: {:?}", err);
                AppError::CollectionNotFound
            })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = collections)]
pub struct InsertedCollection {
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
}

impl InsertedCollection {
    pub async fn insert(&self) -> Result<Collection, AppError> {
        let connection = &mut establish_connection();
        return diesel::insert_into(collections::table)
            .values(self)
            .returning(Collection::as_returning())
            .get_result(connection)
            .map_err(|err| {
                tracing::error!("create collection error: {:?}", err);
                AppError::CreateCollectionFailed
            });
    }
}
