use crate::request::QueryParam;
use ipfs_api_derive::QueryParam;
use log::error;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{error::Error, request::WithForm};

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreCodec {
    #[serde(rename = "dag-cbor")]
    DagCBOR,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputCodec {
    #[serde(rename = "dag-json")]
    DagJSON,
}

#[derive(Debug, Serialize, Deserialize, Default, QueryParam)]
#[serde(rename_all = "kebab-case")]
pub struct PutQuery {
    // Codec that the stored object will be encoded with. Default: dag-cbor. Required: no.
    pub store_codec: Option<StoreCodec>,
    // Codec that the input object is encoded in. Default: dag-json. Required: no.
    pub input_codec: Option<InputCodec>,
    // Pin this object when adding. Required: no.
    pub pin: Option<bool>,
    // Hash function to use. Default: sha2-256. Required: no.
    pub hash: Option<String>,
    // Disable block size check and allow creation of blocks bigger than 1MiB. WARNING: such blocks won't be transferable over the standard bitswap. Default: false. Required: no.
    pub allow_big_block: Option<bool>,
}

pub struct PutRequest {
    pub query: PutQuery,
    pub object_data: serde_json::Value,
}

impl WithForm for PutRequest {
    fn form(&self) -> Result<Form, Error> {
        let form = Form::new().part(
            "object data",
            reqwest::multipart::Part::text(serde_json::to_string(&self.object_data).map_err(
                |err| {
                    error!("Failed to serialize object data: {:?}", err);
                    Error::SerializeObject
                },
            )?),
        );
        Ok(form)
    }
}
