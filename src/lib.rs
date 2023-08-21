use serde::{Deserialize, Serialize};
use url::Url;

const LIVE_URL: &str = "https://api.stytch.com/";
const TEST_URL: &str = "https://test.stytch.com/";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "String")]
pub enum Env {
    Live,
    Test,
    Dev(String),
}

impl From<String> for Env {
    fn from(s: String) -> Self {
        return match s.to_lowercase().as_str() {
            "live" => Env::Live,
            "test" => Env::Test,
            _ => Env::Dev(s),
        };
    }
}

impl Env {
    pub fn base_url(self) -> std::result::Result<Url, url::ParseError> {
        match self {
            Env::Live => Url::parse(LIVE_URL),
            Env::Test => Url::parse(TEST_URL),
            Env::Dev(url) => {
                // The trailing slash is significant in the base URL. Without it, any later joins
                // would drop the last path segment.
                if url.ends_with('/') {
                    Url::parse(&url)
                } else {
                    Url::parse(&(url + "/"))
                }
            }
        }
    }
}

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

#[derive(Clone)]
pub struct Config {
    pub base_url: url::Url,
    pub project_id: String,
    pub secret: String,
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
