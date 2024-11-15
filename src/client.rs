use tokio::sync::OnceCell;
use base64::{engine::general_purpose, Engine as _};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const LIVE_URL: &str = "https://api.stytch.com/";
const TEST_URL: &str = "https://test.stytch.com/";

#[derive(Clone)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
    pub project_id: String,
    jwks_url: String,
    // This would be very natural as a OnceCell, but get_or_try_init is unstable
    // and would require marking this library as only usable with nightly rust.
    // When that feature is stabilized, we should switch to using OnceCell.
    jwks: OnceCell<Jwks>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .finish()
    }
}

pub enum Vertical {
    Consumer,
    B2B,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwk {
    pub kty: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Jwks {
    keys: Vec<Jwk>,
}

impl Client {
    pub fn new(project_id: &str, secret: &str) -> crate::Result<Self> {
        let base_url = Self::base_url(project_id);
        Client::new_with_base_url(project_id, secret, base_url, Vertical::Consumer)
    }

    pub fn new_b2b(project_id: &str, secret: &str) -> crate::Result<Self> {
        let base_url = Self::base_url(project_id);
        Client::new_with_base_url(project_id, secret, base_url, Vertical::B2B)
    }

    pub fn new_with_base_url(
        project_id: &str,
        secret: &str,
        base_url: reqwest::Url,
        vertical: Vertical,
    ) -> crate::Result<Self> {
        let mut headers = http::header::HeaderMap::new();

        let encoded = general_purpose::STANDARD.encode(format!("{}:{}", project_id, secret));
        let basic_auth = format!("Basic {}", encoded).parse::<http::header::HeaderValue>()?;

        headers.insert(http::header::AUTHORIZATION, basic_auth);

        let client = reqwest::Client::builder()
            .user_agent(concat!("stytch-rust ", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        let jwks_url = match vertical {
            Vertical::Consumer => format!("/v1/sessions/jwks/{project_id}"),
            Vertical::B2B => format!("/v1/b2b/sessions/jwks/{project_id}"),
        };

        Ok(Self {
            client,
            project_id: project_id.to_string(),
            base_url,
            jwks_url,
            jwks: OnceCell::new(),
        })
    }

    async fn fetch_jwks(&self) -> crate::Result<Jwks> {
        self.jwks.get_or_try_init(move || async move {
            self
                .send::<_, Jwks>(crate::Request {
                    method: http::Method::GET,
                    path: self.jwks_url.clone(),
                    body: (),
                })
                .await
                .map_err(|_| crate::Error::FetchJwks)
        }).await.map(Jwks::clone)
    }

    pub async fn fetch_jwk(&self, kid: &str) -> crate::Result<Jwk> {
        let jwks = self.fetch_jwks().await?;
        let jwk = jwks
            .keys
            .iter()
            .find(|key| key.kid == kid)
            .ok_or_else(|| crate::Error::JwkNotFound(kid.to_string()))?;
        Ok(jwk.clone())
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
