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

    #[error("Failed to fetch JWKS")]
    FetchJwks,

    #[error("{0:?}")]
    JwkNotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error(transparent)]
    JWTError(#[from] crate::shared::jwt_helpers::JWTError),

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

    #[serde(alias = "error_type", alias = "error")]
    pub error_type: String,
    #[serde(alias = "error_message", alias = "error_description")]
    pub error_message: String,
    #[serde(alias = "error_url", alias = "error_uri")]
    pub error_url: String,
}

pub mod b2b;
pub mod client;
pub mod consumer;
mod shared;
