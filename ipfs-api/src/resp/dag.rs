use serde::{Deserialize, Serialize};

use crate::response::{json, Parsable};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutResponse {
    pub cid: Cid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cid {
    #[serde(rename = "/")]
    pub slash: String,
}

impl Parsable for PutResponse {
    async fn parse(response: reqwest::Response) -> Result<PutResponse, crate::error::Error> {
        json(response)
            .await
            .map(|resp| serde_json::from_value::<PutResponse>(resp).unwrap())
    }
}
