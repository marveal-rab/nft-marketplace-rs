use std::io::Read;

use async_graphql::{Context, Object, Result, SimpleObject, Upload};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Default)]
pub struct FileMutation;

#[derive(Serialize, Deserialize, Debug, Default, SimpleObject)]
pub struct IPFSFile {
    pub url: String,
}

async fn ipfs_file_upload(file_name: String, file_bytes: Vec<u8>) -> Result<String, AppError> {
    let client = Client::new();
    let ipfs_api_endpoint = "http://127.0.0.1:5001/api/v0/add";

    return match client
        .post(ipfs_api_endpoint)
        .multipart(reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes).file_name(file_name),
        ))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(response_body) => {
                        match serde_json::from_str::<serde_json::Value>(&response_body) {
                            Ok(response_body_json) => {
                                let ipfs_url = format!(
                                    "https://ipfs.io/ipfs/{}",
                                    response_body_json["Hash"].as_str().unwrap_or_default()
                                );
                                Ok(ipfs_url.to_string())
                            }
                            Err(err) => {
                                tracing::error!(
                                    "request ipfs add response body deserialize failed: {:?}",
                                    err
                                );
                                Err(AppError::RequestIpfsAddResponseBodyDeserializeFailed)
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!("request ipfs add response no body: {:?}", err);
                        Err(AppError::RequestIpfsAddResponseNoBody)
                    }
                }
            } else {
                Err(AppError::RequestIpfsAddFailed)
            }
        }
        Err(err) => {
            tracing::error!("request ipfs add error: {:?}", err);
            Err(AppError::RequestIpfsAddError)
        }
    };
}

#[Object]
impl FileMutation {
    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> Result<IPFSFile> {
        let file_name = file.value(ctx).unwrap().filename;
        let mut buffer = Vec::new();
        file.value(ctx).unwrap().content.read_to_end(&mut buffer)?;

        match ipfs_file_upload(file_name, buffer).await {
            Ok(url) => Ok(IPFSFile { url }),
            Err(err) => Err(async_graphql::Error::from(err)),
        }
    }
}
