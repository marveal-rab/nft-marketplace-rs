use async_graphql::{Context, Object, SimpleObject, Upload};
use ipfs_api::{AddRequest, Client, LocalIPFSClient};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, util::parse_upload};

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
        let (filename, bytes) = parse_upload(ctx, file)?;
        let add_request = AddRequest::new_with_file(filename, bytes);
        let response = LocalIPFSClient::default()
            .add(add_request)
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
