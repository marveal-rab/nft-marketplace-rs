use async_graphql::{MergedObject, MergedSubscription, Schema};

use crate::domain::file::FileMutation;
use crate::domain::token::{TokenMutation, TokenQuery, TokenSubscription};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TokenQuery);
#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(TokenSubscription);
#[derive(MergedObject, Default)]
pub struct MutationRoot(TokenMutation, FileMutation);

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
