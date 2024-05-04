use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    response::{json, Parsable},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddResponse {
    pub bytes: Option<i64>,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub size: Option<String>,
}

impl Parsable for AddResponse {
    async fn parse(response: reqwest::Response) -> Result<AddResponse, Error> {
        json(response)
            .await
            .map(|resp| serde_json::from_value::<AddResponse>(resp).unwrap())
    }
}
