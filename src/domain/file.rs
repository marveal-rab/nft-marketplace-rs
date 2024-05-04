use std::io::Read;

use async_graphql::{Context, Object, SimpleObject, Upload};
use ipfs_api::{Client, LocalIPFSClient};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

use super::AppResponse;

#[derive(Default)]
pub struct FileMutation;

#[derive(Serialize, Deserialize, Debug, Default, SimpleObject)]
pub struct IPFSFile {
    pub url: String,
    pub hash: String,
}

#[Object]
impl FileMutation {
    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> AppResponse<IPFSFile> {
        let file_name = file.value(ctx).unwrap().filename;
        let mut buffer = Vec::new();
        file.value(ctx)
            .unwrap()
            .content
            .read_to_end(&mut buffer)
            .map_err(|err| {
                tracing::error!("upload file error: {:?}", err);
                AppError::UploadMissingFile
            })?;
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(buffer).file_name(file_name),
        );
        let response = LocalIPFSClient::default()
            .add_multipart(form)
            .await
            .map_err(|err| {
                tracing::error!("upload file to ipfs error: {:?}", err);
                AppError::RequestIpfsFailed
            })?;
        response
            .hash
            .map(|hash| {
                Some(IPFSFile {
                    hash: hash.clone(),
                    url: format!("http://127.0.0.1:8080/ipfs/{}", hash.clone()),
                })
            })
            .ok_or(AppError::HashMismatch)
    }
}
