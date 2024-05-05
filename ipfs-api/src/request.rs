use log::error;
use reqwest::{multipart::Form, Client, Response, Url};
use serde::Serialize;

use crate::error::Error;

pub trait QueryParam: Serialize + Send + Default {
    fn encode(&self) -> String {
        serde_urlencoded::to_string(self).unwrap()
    }
}

pub trait Request {
    fn path(&self) -> String;
}

pub struct RequestUrl<'a, Q>
where
    Q: QueryParam,
{
    pub base: &'a Url,
    pub path: &'a str,
    pub query: &'a Q,
}

impl<'a, Q: QueryParam> RequestUrl<'a, Q> {
    pub fn url(&self) -> Result<Url, Error> {
        let url = format!("{}/{}?{}", self.base, self.path, self.query.encode());
        Url::parse(&url).map_err(|err| {
            error!("Failed to parse URL: {:?}", err);
            Error::UrlParse
        })
    }

    pub fn plain_url(&self) -> Result<String, Error> {
        self.url().map(|url| url.to_string())
    }
}

pub async fn post_with_form(url: Url, form: Form) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .request(reqwest::Method::POST, url)
        .multipart(form)
        .send()
        .await
        .map_err(|err| {
            error!("Failed to send request: {:?}", err);
            Error::RequestError
        })?;
    if !response.status().is_success() {
        return Err(Error::RequestFailed);
    }
    Ok(response)
}

pub async fn post(url: Url) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .request(reqwest::Method::POST, url)
        .send()
        .await
        .map_err(|err| {
            error!("Failed to send request: {:?}", err);
            Error::RequestError
        })?;
    if !response.status().is_success() {
        return Err(Error::RequestFailed);
    }
    Ok(response)
}

impl<'a, Q> RequestUrl<'a, Q>
where
    Q: QueryParam,
{
    pub fn new(base: &'a Url, path: &'a str, query: &'a Q) -> Self {
        RequestUrl { base, path, query }
    }
}

pub trait WithForm {
    fn form(&self) -> Result<Form, Error>;
}
