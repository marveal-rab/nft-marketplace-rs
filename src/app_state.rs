use async_graphql::{EmptyMutation, Schema};

use crate::domain::models::{QueryRoot, SubscriptionRoot};

pub type SchemaRoot = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

#[derive(Clone)]
pub struct AppState {
    pub schema: SchemaRoot,
}

impl AppState {
    pub fn new() -> Self {
        let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);
        Self { schema }
    }
}
