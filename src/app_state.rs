use async_graphql::{MergedObject, MergedSubscription, Schema};

use crate::domain::collection::{CollectionMutation, CollectionQuery};
use crate::domain::file::FileMutation;
use crate::domain::nft::{NFTMutation, NFTQuery};
use crate::domain::token::{TokenMutation, TokenQuery, TokenSubscription};
use crate::domain::user::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TokenQuery, CollectionQuery, UserQuery, NFTQuery);
#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(TokenSubscription);
#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TokenMutation,
    FileMutation,
    UserMutation,
    CollectionMutation,
    NFTMutation,
);

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

#[derive(Clone)]
pub struct AppState {
    pub schema: SchemaRoot,
}

impl AppState {
    pub fn new() -> Self {
        let schema = Schema::new(
            QueryRoot::default(),
            MutationRoot::default(),
            SubscriptionRoot::default(),
        );
        Self { schema }
    }
}
