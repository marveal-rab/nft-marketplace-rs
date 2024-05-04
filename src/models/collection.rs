use core::str;

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
    pub contract_address: String,
    pub chain_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Default)]
pub struct CollectionQuery {
    pub id: Option<Uuid>,
    pub owner: Option<String>,
    pub contract_address: Option<String>,
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

    pub async fn find_by_query(query: CollectionQuery) -> Result<Option<Collection>, AppError> {
        let connection = &mut establish_connection();
        let mut query_builder = collections::table.into_boxed();
        if let Some(_id) = query.id {
            query_builder = query_builder.filter(collections::id.eq(_id));
        }
        if let Some(owner) = query.owner {
            query_builder = query_builder.filter(collections::owner.eq(owner));
        }
        if let Some(contract_address) = query.contract_address {
            query_builder =
                query_builder.filter(collections::contract_address.eq(contract_address));
        }
        match query_builder
            .select(Collection::as_select())
            .limit(1)
            .first(connection)
            .optional()
        {
            Ok(collection) => Ok(collection),
            Err(err) => {
                if err == diesel::NotFound {
                    Ok(None)
                } else {
                    tracing::error!("find collection error: {:?}", err);
                    Err(AppError::CollectionQueryError)
                }
            }
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = collections)]
pub struct InsertedCollection {
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
    pub contract_address: String,
    pub chain_id: i32,
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
