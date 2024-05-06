use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::errors::AppError;

use super::schema::nft_traits;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = nft_traits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NFTTrait {
    pub id: uuid::Uuid,
    pub nft_id: uuid::Uuid,
    pub trait_type: String,
    pub trait_value: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl NFTTrait {
    pub async fn list_by_nft_id(nft_id: uuid::Uuid) -> Result<Vec<NFTTrait>, AppError> {
        let connection = &mut super::establish_connection();
        return nft_traits::table
            .filter(nft_traits::nft_id.eq(nft_id))
            .get_results(connection)
            .map_err(|err| {
                tracing::error!("find nft trait by nft_id error: {:?}", err);
                AppError::NftTraitNotFound
            });
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = nft_traits)]
pub struct InsertedNFTTrait {
    pub nft_id: uuid::Uuid,
    pub trait_type: String,
    pub trait_value: String,
}

pub struct BatchInsertedNFTTrait {
    pub traits: Vec<InsertedNFTTrait>,
}

impl InsertedNFTTrait {
    pub async fn insert(&self) -> Result<NFTTrait, AppError> {
        let connection = &mut super::establish_connection();
        return diesel::insert_into(nft_traits::table)
            .values(self)
            .get_result(connection)
            .map_err(|err| {
                tracing::error!("create nft trait error: {:?}", err);
                AppError::CreateNFTTraitFailed
            });
    }
}

impl BatchInsertedNFTTrait {
    pub async fn insert(&self) -> Result<Vec<NFTTrait>, AppError> {
        if self.traits.is_empty() {
            return Ok(vec![]);
        }

        let connection = &mut super::establish_connection();
        return diesel::insert_into(nft_traits::table)
            .values(&self.traits)
            .get_results(connection)
            .map_err(|err| {
                tracing::error!("create nft trait error: {:?}", err);
                AppError::CreateNFTTraitFailed
            });
    }
}
