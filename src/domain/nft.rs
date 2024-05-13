use async_graphql::{Context, InputObject, Object, SimpleObject};
use ipfs_api::client::{Client, LocalIPFSClient};
use ipfs_api::req::files::{WriteQuery, WriteRequest};
use serde::{Deserialize, Serialize};

use crate::models;
use crate::{
    errors::AppError,
    models::{
        collection::{Collection, CollectionQuery},
        nft::{InsertedNFT, NFT},
        nft_trait::{BatchInsertedNFTTrait, InsertedNFTTrait, NFTTrait},
    },
};

use super::{token::Token, AppResponse};

#[derive(Default)]
pub struct NFTMutation;
#[derive(Default)]
pub struct NFTQuery;

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct NewNFT {
    pub token_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub supply: i32,
    pub external_link: Option<String>,
    pub collection: String,
    pub traits: Option<Vec<NewNFTTrait>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
pub struct NewNFTTrait {
    pub trait_type: String,
    pub trait_value: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct NFTResult {
    pub token_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub supply: i32,
    pub external_link: Option<String>,
    pub owner: String,
    pub collection: String,
    pub traits: Option<Vec<NFTTraitResult>>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct NFTTraitResult {
    pub trait_type: String,
    pub trait_value: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub fn convert_to_inserted_nft(new_nft: &NewNFT, owner: String) -> InsertedNFT {
    InsertedNFT {
        token_id: new_nft.token_id,
        name: new_nft.name.clone(),
        description: new_nft.description.clone(),
        image_url: new_nft.image_url.clone(),
        supply: new_nft.supply,
        external_link: new_nft.external_link.clone(),
        owner,
        collection: new_nft.collection.clone(),
    }
}

pub fn convert_to_batch_inserted_nft_trait(
    new_nft: &NewNFT,
    nft_id: &uuid::Uuid,
) -> BatchInsertedNFTTrait {
    if new_nft.traits.is_none() {
        return BatchInsertedNFTTrait { traits: vec![] };
    }
    let traits = new_nft
        .traits
        .as_ref()
        .unwrap()
        .into_iter()
        .map(|trait_item| InsertedNFTTrait {
            nft_id: nft_id.clone(),
            trait_type: trait_item.trait_type.clone(),
            trait_value: trait_item.trait_value.clone(),
        })
        .collect();
    BatchInsertedNFTTrait { traits }
}

pub fn convert_to_nft_result(nft: &NFT, nft_traits: &Vec<NFTTrait>) -> Option<NFTResult> {
    Some(NFTResult {
        token_id: nft.token_id,
        name: nft.name.clone(),
        description: nft.description.clone(),
        image_url: nft.image_url.clone(),
        supply: nft.supply,
        external_link: nft.external_link.clone(),
        owner: nft.owner.clone(),
        collection: nft.collection.clone(),
        traits: Some(
            nft_traits
                .into_iter()
                .map(|trait_item| NFTTraitResult {
                    trait_type: trait_item.trait_type.clone(),
                    trait_value: trait_item.trait_value.clone(),
                    created_at: trait_item.created_at,
                    updated_at: trait_item.updated_at,
                })
                .collect(),
        ),
        created_at: nft.created_at,
        updated_at: nft.updated_at,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub dna: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub external_link: Option<String>,
    pub traits: Option<Vec<NFTTraitMetadata>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTTraitMetadata {
    pub trait_type: String,
    pub value: String,
}

pub fn convert_to_nft_metadata(nft: &NFT, nft_traits: &Vec<NFTTrait>) -> NFTMetadata {
    NFTMetadata {
        dna: nft.id,
        name: nft.name.clone(),
        description: nft.description.clone(),
        image_url: nft.image_url.clone(),
        external_link: nft.external_link.clone(),
        traits: Some(
            nft_traits
                .into_iter()
                .map(|trait_item| NFTTraitMetadata {
                    trait_type: trait_item.trait_type.clone(),
                    value: trait_item.trait_value.clone(),
                })
                .collect(),
        ),
    }
}

#[Object]
impl NFTMutation {
    async fn create_nft(&self, ctx: &Context<'_>, new_nft: NewNFT) -> AppResponse<NFTResult> {
        // Check if the user is authenticated
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;

        // check collection is valid
        let collection_query = CollectionQuery {
            owner: Some(encrypt_user_info.address.clone()),
            contract_address: Some(new_nft.collection.clone()),
            ..Default::default()
        };
        let collection = Collection::find_by_query(collection_query)
            .await?
            .ok_or(AppError::CollectionNotFound)?;

        // Create a new NFT
        let nft = convert_to_inserted_nft(&new_nft, encrypt_user_info.address)
            .insert()
            .await?;
        // Create a new NFT trait
        let nft_traits = convert_to_batch_inserted_nft_trait(&new_nft, &nft.id)
            .insert()
            .await?;

        // upload nft metadata to IPFS
        let filename = format!("{}.json", nft.token_id);
        let path = format!("/{}/{}.json", collection.dir_name.clone(), nft.token_id);
        let nft_metadata = convert_to_nft_metadata(&nft, &nft_traits);
        let write_request = WriteRequest {
            query: WriteQuery::new_with_arg(path),
            bytes: serde_json::to_vec(&nft_metadata).unwrap(),
            filename,
        };
        let _ = LocalIPFSClient::default()
            .files_write(write_request)
            .await
            .map_err(|err| {
                tracing::error!("IPFS write error: {:?}", err);
                AppError::RequestIpfsError
            })?;
        Ok(convert_to_nft_result(&nft, &nft_traits))
    }
}

#[Object]
impl NFTQuery {
    async fn nft(&self, ctx: &Context<'_>, token_id: i32) -> AppResponse<NFTResult> {
        // Check if the user is authenticated
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        // Find NFT by token_id
        let nft = NFT::find_by_token_id(token_id).await?;
        // Find NFT traits by nft_id
        let nft_traits = NFTTrait::list_by_nft_id(nft.id).await?;

        Ok(convert_to_nft_result(&nft, &nft_traits))
    }

    async fn next_token_id(&self, ctx: &Context<'_>, contract_address: String) -> AppResponse<i64> {
        // Check if the user is authenticated
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;

        // check collection is valid
        let collection_query = CollectionQuery {
            owner: Some(encrypt_user_info.address.clone()),
            contract_address: Some(contract_address),
            ..Default::default()
        };
        let collection = Collection::find_by_query(collection_query)
            .await?
            .ok_or(AppError::CollectionNotFound)?;

        // Find NFT count by collection
        let count = models::nft::NFTQuery {
            collection: Some(collection.contract_address),
            ..Default::default()
        }
        .count()
        .await?;
        Ok(Some(count + 1))
    }
}
