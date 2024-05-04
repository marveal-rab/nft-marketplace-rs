use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("request url query encode error")]
    UrlEncode,
    #[error("request url parse error")]
    UrlParse,

    #[error("request failed")]
    RequestFailed,
    #[error("request error")]
    RequestError,
    #[error("response body read error")]
    ResponseBodyReadError,
    #[error("response body serialize error")]
    ResponseBodySerializeError,
}
