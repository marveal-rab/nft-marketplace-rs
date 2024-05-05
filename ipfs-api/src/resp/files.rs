use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    response::{json, Parsable},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FlushResponse {
    pub cid: String,
}

impl Parsable for FlushResponse {
    async fn parse(response: reqwest::Response) -> Result<FlushResponse, Error> {
        json(response)
            .await
            .map(|resp| serde_json::from_value::<FlushResponse>(resp).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LsResponse {
    pub entries: Vec<LsObject>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LsObject {
    pub hash: String,
    pub name: String,
    pub size: i64,
    #[serde(rename = "Type")]
    pub typ: i32,
}

impl Parsable for LsResponse {
    async fn parse(response: reqwest::Response) -> Result<LsResponse, Error> {
        json(response)
            .await
            .map(|resp| serde_json::from_value::<LsResponse>(resp).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StatResponse {
    pub blocks: i32,
    pub cumulative_size: u64,
    pub hash: String,
    pub local: bool,
    pub size: u64,
    pub size_local: u64,
    #[serde(rename = "Type")]
    pub typ: String,
    pub with_locality: bool,
}

impl Parsable for StatResponse {
    async fn parse(response: reqwest::Response) -> Result<StatResponse, Error> {
        json(response)
            .await
            .map(|resp| serde_json::from_value::<StatResponse>(resp).unwrap())
    }
}
