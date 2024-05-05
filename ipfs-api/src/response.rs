use log::error;

use crate::error::Error;

pub async fn json(response: reqwest::Response) -> Result<serde_json::Value, Error> {
    let response_body = response.text().await.map_err(|err| {
        error!("Failed to read response body: {:?}", err);
        Error::ResponseBodyReadError
    })?;
    serde_json::from_str::<serde_json::Value>(&response_body).map_err(|err| {
        error!("Failed to parse response body: {:?}", err);
        Error::ResponseBodySerializeError
    })
}

pub async fn text(response: reqwest::Response) -> Result<String, Error> {
    response.text().await.map_err(|err| {
        error!("Failed to read response body: {:?}", err);
        Error::ResponseBodyReadError
    })
}

pub trait Parsable {
    async fn parse(response: reqwest::Response) -> Result<impl Parsable, Error>;
}

pub struct EmptyResponse;

impl Parsable for EmptyResponse {
    async fn parse(_response: reqwest::Response) -> Result<Self, Error> {
        let _ = text(_response).await?;
        Ok(EmptyResponse)
    }
}
