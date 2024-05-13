use async_graphql::{Context, Object, SimpleObject, Upload};
use ipfs_api::client::{Client, LocalIPFSClient};
use ipfs_api::req::{
    add::AddRequest,
    files::{MkdirQuery, MkdirRequest, StatQuery, StatRequest},
};
use ipfs_api::resp::add::AddResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, util::parse_upload};

use super::token::Token;
use super::AppResponse;

#[derive(Default)]
pub struct FileMutation;

#[derive(Serialize, Deserialize, Debug, Default, SimpleObject)]
pub struct IPFSFile {
    pub url: String,
    pub hash: String,
}

impl IPFSFile {
    pub fn new(hash: &String) -> Self {
        Self {
            hash: hash.clone(),
            url: generate_url_by_hash(hash),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, SimpleObject)]
pub struct IPFSFileStat {
    pub name: String,
    pub hash: String,
    pub url: String,
}

impl IPFSFileStat {
    pub fn new(name: &String, hash: &String) -> Self {
        Self {
            name: name.clone(),
            hash: hash.clone(),
            url: generate_url_by_hash(hash),
        }
    }
}

pub fn generate_url_by_hash(hash: &String) -> String {
    format!("http://127.0.0.1:8080/ipfs/{}", hash.clone())
}

#[Object]
impl FileMutation {
    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> AppResponse<IPFSFile> {
        // Check if the user is authenticated
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;
        let (filename, bytes) = parse_upload(ctx, file)?;
        let add_request = AddRequest::new_with_file(filename, bytes);
        let response: AddResponse =
            LocalIPFSClient::default()
                .add(add_request)
                .await
                .map_err(|err| {
                    tracing::error!("upload file to ipfs error: {:?}", err);
                    AppError::RequestIpfsFailed
                })?;
        response
            .hash
            .map(|hash| Some(IPFSFile::new(&hash)))
            .ok_or(AppError::HashMismatch)
    }

    async fn files_mkdir(&self, ctx: &Context<'_>) -> AppResponse<IPFSFileStat> {
        // Check if the user is authenticated
        let encrypt_user_info = ctx
            .data_opt::<Token>()
            .ok_or(AppError::MissingCredentials)?
            .parse()?;

        // generate dir name
        let arg = Uuid::new_v4().to_string();
        // mkdir
        let mkdir_request = MkdirRequest {
            query: MkdirQuery::new_with_arg(&arg),
        };
        let _ = LocalIPFSClient::default()
            .files_mkdir(mkdir_request)
            .await
            .map_err(|err| {
                tracing::error!("mkdir to ipfs error: {:?}", err);
                AppError::RequestIpfsFailed
            })?;
        // get dir hash
        let stat_request = StatRequest {
            query: StatQuery::new_with_arg(&arg),
        };
        let response = LocalIPFSClient::default()
            .files_stat(stat_request)
            .await
            .map_err(|err| {
                tracing::error!("ipfs files stat error: {:?}", err);
                AppError::RequestIpfsFailed
            })?;
        response
            .hash
            .map(|hash| Some(IPFSFileStat::new(&arg, &hash)))
            .ok_or(AppError::HashMismatch)
    }
}
