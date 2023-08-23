use derivative::Derivative;
use serde::{de::DeserializeOwned, Serialize};

const LIVE_URL: &str = "https://api.stytch.com/";
const TEST_URL: &str = "https://test.stytch.com/";

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Client {
    #[derivative(Debug = "ignore")]
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
}

impl Client {
    pub fn new(project_id: String, secret: String) -> crate::Result<Self> {
        let base_url = Self::base_url(&project_id);
        Client::new_with_base_url(project_id, secret, base_url)
    }

    pub fn new_with_base_url(
        project_id: String,
        secret: String,
        base_url: reqwest::Url,
    ) -> crate::Result<Self> {
        let mut headers = http::header::HeaderMap::new();

        let encoded = base64::encode(format!("{}:{}", project_id, secret));
        let basic_auth = format!("Basic {}", encoded).parse::<http::header::HeaderValue>()?;

        headers.insert(http::header::AUTHORIZATION, basic_auth);

        let client = reqwest::Client::builder()
            .user_agent(concat!("stytch-rust ", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        Ok(Self { client, base_url })
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

    fn base_url(project_id: &str) -> reqwest::Url {
        if project_id.starts_with("project-live-") {
            reqwest::Url::parse(LIVE_URL).unwrap()
        } else {
            reqwest::Url::parse(TEST_URL).unwrap()
        }
    }
}

impl From<reqwest::Error> for crate::Error {
    fn from(err: reqwest::Error) -> crate::Error {
        crate::Error::Other(Box::new(err))
    }
}
