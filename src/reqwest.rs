use derivative::Derivative;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Client {
    #[derivative(Debug = "ignore")]
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
}

impl Client {
    pub fn new(config: crate::Config) -> crate::Result<Self> {
        let mut headers = http::header::HeaderMap::new();

        let encoded = base64::encode(format!("{}:{}", config.project_id, config.secret));
        let basic_auth = format!("Basic {}", encoded)
            .parse::<http::header::HeaderValue>()?;

        headers.insert(http::header::AUTHORIZATION, basic_auth);

        let client = reqwest::Client::builder()
            .user_agent(concat!("stytch-rust ", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: config.base_url,
        })
    }

    pub async fn send<Req, Res>(&self, req: crate::Request<Req>) -> crate::Result<Res>
    where
        Req: Serialize + std::fmt::Debug + std::marker::Send,
        Res: DeserializeOwned + std::fmt::Debug,
    {
        let url = self.base_url.join(&req.path)?;
        let req = self.client.request(req.method, url).json(&req.body);

        tracing::debug!({ req = ?req }, "send Stytch request");
        let res = req.send().await?;
        if res.status().is_success() {
            let body = res.json().await?;
            tracing::debug!({ ?body }, "Stytch response success");
            Ok(body)
        } else {
            let err = res.json::<crate::ErrorResponse>().await?;
            tracing::debug!({ ?err }, "Stytch response error");
            Err(crate::Error::Response(err))
        }
    }
}

impl From<reqwest::Error> for crate::Error {
    fn from(err: reqwest::Error) -> crate::Error {
        crate::Error::Other(Box::new(err))
    }
}
