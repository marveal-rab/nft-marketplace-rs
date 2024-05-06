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
            log::error!("Failed to parse URL: {:?}", err);
            Error::UrlParse
        })
    }

    pub fn plain_url(&self) -> Result<String, Error> {
        self.url().map(|url| url.to_string())
    }
}

pub async fn post_with_form(url: Url, form: Form) -> Result<Response, Error> {
    _post(url, Some(form)).await
}

pub async fn post(url: Url) -> Result<Response, Error> {
    _post(url, None).await
}

async fn _post(url: Url, form: Option<Form>) -> Result<Response, Error> {
    let client = Client::new();
    let mut request_builder = client.request(reqwest::Method::POST, url.clone());
    if let Some(form) = form {
        request_builder = request_builder.multipart(form);
    }
    let response = request_builder.send().await.map_err(|err| {
        log::error!(
            "Send request get error: \n\turl:{:?}\n\terror:{:?}",
            url.clone().to_string(),
            err
        );
        Error::RequestError
    })?;
    if !response.status().is_success() {
        log::error!(
            "send request failed: \n\turl:{:?}\n\tresponse:{:?}",
            url.clone().to_string(),
            response.text().await.unwrap_or_default()
        );
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
