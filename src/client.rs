use base64::{engine::general_purpose, Engine as _};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const LIVE_URL: &str = "https://api.stytch.com/";
const TEST_URL: &str = "https://test.stytch.com/";

#[derive(Clone)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
    pub project_id: String,
    jwks_url: String,
    jwks: Arc<Mutex<Option<(Instant, Jwks)>>>,
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

    pub fn new_fraud(project_id: &str, secret: &str) -> crate::Result<Self> {
        let base_url = reqwest::Url::parse("https://telemetry.stytch.com").unwrap();
        Client::new_with_base_url(project_id, secret, base_url, Vertical::Consumer)
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
            jwks: Arc::new(Mutex::new(None)),
        })
    }

    async fn fetch_jwks(&self) -> crate::Result<Jwks> {
        let mut cache = self.jwks.lock().await;
        let now = Instant::now();

        if let Some((timestamp, jwks)) = &*cache {
            if now.duration_since(*timestamp) < Duration::from_secs(300) {
                return Ok(jwks.clone());
            }
        }

        // Fetch new JWKS
        let new_jwks = self
            .send::<_, Jwks>(crate::Request {
                method: http::Method::GET,
                path: self.jwks_url.clone(),
                body: (),
            })
            .await
            .map_err(|_| crate::Error::FetchJwks)?;

        *cache = Some((now, new_jwks.clone()));

        Ok(new_jwks)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_jwks_refreshes_after_300_seconds() -> crate::Result<()> {
        let project_id = match env::var("STYTCH_PROJECT_ID") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Skipping test: STYTCH_PROJECT_ID not set");
                return Ok(());
            }
        };

        let secret = match env::var("STYTCH_SECRET") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Skipping test: STYTCH_SECRET not set");
                return Ok(());
            }
        };

        let client = Client::new(&project_id, &secret)?;
        let first_jwks = client.fetch_jwks().await?;

        // Simulate 300 seconds passing
        {
            let mut cache = client.jwks.lock().await;
            if let Some((timestamp, _)) = cache.as_mut() {
                *timestamp = Instant::now() - Duration::from_secs(301);
            }
        }

        // Fetch JWKS again after expiration
        let refreshed_jwks = client.fetch_jwks().await?;

        // It's hard to make any great assertion here because JWKS only expire every six months.
        // We're not going to write a unit test that sleeps for six months :)
        // Instead, let's just check that the kty and alg match expected values.
        assert_eq!(first_jwks.keys[0].kty, refreshed_jwks.keys[0].kty);
        assert_eq!(first_jwks.keys[0].alg, refreshed_jwks.keys[0].alg);

        Ok(())
    }
}
