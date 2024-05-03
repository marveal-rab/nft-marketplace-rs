use async_graphql::{Context, InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    models::collection::{Collection, InsertedCollection},
};

use super::token::Token;

#[derive(Default)]
pub struct CollectionMutation;
#[derive(Default)]
pub struct CollectionQuery;

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct NewCollection {
    pub name: String,
    pub symbol: String,
    pub pic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CreateCollectionResult {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
}

impl From<Collection> for CreateCollectionResult {
    fn from(collection: Collection) -> Self {
        Self {
            id: collection.id.to_string(),
            name: collection.name,
            symbol: collection.symbol,
            owner: collection.owner,
            pic_url: collection.pic_url,
        }
    }
}

#[Object]
impl CollectionMutation {
    pub async fn create_collection<'a>(
        &self,
        ctx: &Context<'_>,
        new_collection: NewCollection,
    ) -> Result<CreateCollectionResult, AppError> {
        tracing::info!("Creating collection: {:?}", new_collection);
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        tracing::info!(
            "Creating collection for user: {}",
            encrypt_user_info.address
        );
        InsertedCollection {
            name: new_collection.name,
            symbol: new_collection.symbol,
            owner: encrypt_user_info.address,
            pic_url: new_collection.pic_url,
        }
        .insert()
        .await
        .map(|collection| CreateCollectionResult::from(collection))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CollectionResult {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
}

impl From<Collection> for CollectionResult {
    fn from(collection: Collection) -> Self {
        Self {
            id: collection.id.to_string(),
            name: collection.name,
            symbol: collection.symbol,
            owner: collection.owner,
            pic_url: collection.pic_url,
        }
    }
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct ListCollectionInput {
    pub owner: String,
}

#[Object]
impl CollectionQuery {
    pub async fn list_collections_for_owner<'a>(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CollectionResult>, AppError> {
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        let collections = Collection::find_by_owner(encrypt_user_info.address).await?;
        Ok(collections
            .into_iter()
            .map(CollectionResult::from)
            .collect())
    }
}
