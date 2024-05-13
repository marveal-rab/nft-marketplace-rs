use std::future::Future;

use reqwest::Url;

use crate::{
    error::Error,
    req,
    request::{post, post_with_form, RequestUrl, WithForm},
    resp::{self, add::AddResponse},
    response::{EmptyResponse, Parsable},
};

pub trait Client {
    fn add(
        &self,
        req: req::add::AddRequest,
    ) -> impl Future<Output = Result<AddResponse, Error>> + Send;

    /**
     * Make directories.
     */
    fn files_mkdir(
        &self,
        req: req::files::MkdirRequest,
    ) -> impl Future<Output = Result<EmptyResponse, Error>> + Send;

    /**
     * Flush a given path's data to disk.
     */
    fn files_flush(
        &self,
        req: req::files::FlushRequest,
    ) -> impl Future<Output = Result<resp::files::FlushResponse, Error>> + Send;

    /**
     * Append to (modify) a file in MFS.
     */
    fn files_write(
        &self,
        req: req::files::WriteRequest,
    ) -> impl Future<Output = Result<EmptyResponse, Error>> + Send;

    fn files_stat(
        &self,
        req: req::files::StatRequest,
    ) -> impl Future<Output = Result<resp::files::StatResponse, Error>> + Send;

    /**
     * Add a DAG node to IPFS.
     */
    fn dag_put(
        &self,
        req: req::dag::PutRequest,
    ) -> impl Future<Output = Result<resp::dag::PutResponse, Error>> + Send;
}

pub struct LocalIPFSClient {
    endpoint: Url,
}

impl LocalIPFSClient {
    #[allow(dead_code)]
    fn new(endpoint: String) -> Self {
        LocalIPFSClient {
            endpoint: Url::parse(&endpoint).unwrap(),
        }
    }
}

impl Default for LocalIPFSClient {
    fn default() -> Self {
        LocalIPFSClient {
            endpoint: Url::parse("http://127.0.0.1:5001/api/v0").unwrap(),
        }
    }
}

impl Client for LocalIPFSClient {
    async fn add(&self, req: req::add::AddRequest) -> Result<AddResponse, Error> {
        let url = RequestUrl::new(&self.endpoint, "add", &req.query).url()?;
        let form = req.form()?;
        let response = post_with_form(url, form).await?;
        AddResponse::parse(response).await
    }

    async fn files_mkdir(&self, req: req::files::MkdirRequest) -> Result<EmptyResponse, Error> {
        let url = RequestUrl::new(&self.endpoint, "files/mkdir", &req.query).url()?;
        let response = post(url).await?;
        EmptyResponse::parse(response).await
    }

    async fn files_flush(
        &self,
        req: req::files::FlushRequest,
    ) -> Result<resp::files::FlushResponse, Error> {
        let url: Url = RequestUrl::new(&self.endpoint, "files/flush", req.query).url()?;
        let response = post(url).await?;
        resp::files::FlushResponse::parse(response).await
    }

    async fn files_write(&self, req: req::files::WriteRequest) -> Result<EmptyResponse, Error> {
        let url: Url = RequestUrl::new(&self.endpoint, "files/write", &req.query).url()?;
        let form = req.form()?;
        let response = post_with_form(url, form).await?;
        // Add Test Log
        log::info!("files_write response: {:?}", response);
        EmptyResponse::parse(response).await
    }

    async fn dag_put(&self, req: req::dag::PutRequest) -> Result<resp::dag::PutResponse, Error> {
        let url: Url = RequestUrl::new(&self.endpoint, "dag/put", &req.query).url()?;
        let form = req.form()?;
        let response = post_with_form(url, form).await?;
        resp::dag::PutResponse::parse(response).await
    }

    async fn files_stat(
        &self,
        req: req::files::StatRequest,
    ) -> Result<resp::files::StatResponse, Error> {
        let url: Url = RequestUrl::new(&self.endpoint, "files/stat", &req.query).url()?;
        let response = post(url).await?;
        // Add Test Log
        log::info!("files_stat response: {:?}", response);
        resp::files::StatResponse::parse(response).await
    }
}
