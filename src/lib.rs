use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0:?}")]
    Response(ErrorResponse),

    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Request<Body: Serialize + Send> {
    pub method: http::Method,
    pub path: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub error_type: String,
    pub error_message: String,
    pub error_url: String,
}

#[cfg(feature = "reqwest")]
pub mod reqwest;

pub mod b2b;
pub mod consumer;
