use log::error;
use reqwest::{multipart::Form, Client, Response, Url};
use serde::Serialize;

use crate::error::Error;

pub trait Request: Serialize + Send {
    const PATH: &'static str;
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn url(&self, endpoint: &Url) -> Result<Url, Error> {
        let url = format!(
            "{}/{}?{}",
            endpoint,
            Self::PATH,
            serde_urlencoded::to_string(self).map_err(|err| {
                error!("Failed to encode URL: {:?}", err);
                Error::UrlEncode
            })?,
        );
        Url::parse(&url).map_err(|err| {
            error!("Failed to parse URL: {:?}", err);
            Error::UrlParse
        })
    }

    fn plain_url(&self, endpoint: &Url) -> Result<String, Error> {
        self.url(endpoint).map(|url| url.to_string())
    }

    async fn do_request_with_form(&self, endpoint: &Url, form: Form) -> Result<Response, Error> {
        let client = Client::new();
        let url = self.url(endpoint)?;
        let response = client
            .request(Self::METHOD, url)
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
}
