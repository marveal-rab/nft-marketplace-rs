use async_graphql::{Context, InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    domain::AppResponse,
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
    pub contract_address: String,
    pub chain_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CreateCollectionResult {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub owner: String,
    pub pic_url: String,
    pub contract_address: String,
    pub chain_id: i32,
}

impl From<Collection> for CreateCollectionResult {
    fn from(collection: Collection) -> Self {
        Self {
            id: collection.id.to_string(),
            name: collection.name,
            symbol: collection.symbol,
            owner: collection.owner,
            pic_url: collection.pic_url,
            contract_address: collection.contract_address,
            chain_id: collection.chain_id,
        }
    }
}

impl InsertedCollection {
    pub fn new(new_collection: NewCollection, owner: String) -> Self {
        Self {
            name: new_collection.name,
            symbol: new_collection.symbol,
            owner,
            pic_url: new_collection.pic_url,
            contract_address: new_collection.contract_address,
            chain_id: new_collection.chain_id,
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
        InsertedCollection::new(new_collection, encrypt_user_info.address)
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
    pub contract_address: String,
    pub chain_id: i32,
}

impl From<Collection> for CollectionResult {
    fn from(collection: Collection) -> Self {
        Self {
            id: collection.id.to_string(),
            name: collection.name,
            symbol: collection.symbol,
            owner: collection.owner,
            pic_url: collection.pic_url,
            contract_address: collection.contract_address,
            chain_id: collection.chain_id,
        }
    }
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct FindCollectionInput {
    pub collection_address: Option<String>,
}

#[Object]
impl CollectionQuery {
    pub async fn list_collections_for_owner(
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

    pub async fn find_collection_for_owner(
        &self,
        ctx: &Context<'_>,
        input: FindCollectionInput,
    ) -> AppResponse<CollectionResult> {
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        let collection_query = crate::models::collection::CollectionQuery {
            owner: Some(encrypt_user_info.address),
            contract_address: input.collection_address,
            ..Default::default()
        };
        Collection::find_by_query(collection_query)
            .await
            .map(|collection| collection.map(CollectionResult::from))
    }
}
