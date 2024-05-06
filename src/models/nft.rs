use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::errors::AppError;

use super::schema::nfts;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = nfts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NFT {
    pub id: uuid::Uuid,
    pub token_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub supply: i32,
    pub external_link: Option<String>,
    pub owner: String,
    pub collection: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl NFT {
    pub async fn find_by_token_id(token_id: i32) -> Result<NFT, AppError> {
        let connection = &mut super::establish_connection();
        return nfts::table
            .filter(nfts::token_id.eq(token_id))
            .first(connection)
            .map_err(|err| {
                tracing::error!("find nft by token_id error: {:?}", err);
                AppError::NftNotFound
            });
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = nfts)]
pub struct InsertedNFT {
    pub token_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub supply: i32,
    pub external_link: Option<String>,
    pub owner: String,
    pub collection: String,
}

impl InsertedNFT {
    pub async fn insert(&self) -> Result<NFT, AppError> {
        let connection = &mut super::establish_connection();
        return diesel::insert_into(nfts::table)
            .values(self)
            .get_result(connection)
            .map_err(|err| {
                tracing::error!("create nft error: {:?}", err);
                AppError::CreateNFTFailed
            });
    }
}
