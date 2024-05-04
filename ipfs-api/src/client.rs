use std::future::Future;

use reqwest::{multipart::Form, Url};

use crate::{
    error::Error, req::add::AddRequest, request::Request, resp::add::AddResponse,
    response::Parsable,
};

pub trait Client {
    fn add_multipart(&self, form: Form) -> impl Future<Output = Result<AddResponse, Error>> + Send;
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
            endpoint: Url::parse("http://http://127.0.0.1:5001/api/v0").unwrap(),
        }
    }
}

impl Client for LocalIPFSClient {
    async fn add_multipart(&self, form: Form) -> Result<AddResponse, Error> {
        let request = AddRequest::default();
        let response = request.do_request_with_form(&self.endpoint, form).await?;
        AddResponse::parse(response).await
    }
}
