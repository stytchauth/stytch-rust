use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum JWTError {
    #[error("JWT too old")]
    TooOld,

    #[error("JWT missing '{0}' field")]
    MissingField(String),

    #[error(transparent)]
    Decode(#[from] jsonwebtoken::errors::Error),

    #[error("Payload format invalid")]
    PayloadFormat,

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericClaims {
    pub reserved_claims: HashMap<String, Value>,
    pub untyped_claims: HashMap<String, Value>,
}

pub async fn authenticate_jwt(
    http_client: &crate::client::Client,
    jwt: &str,
    max_token_age_seconds: Option<u64>,
) -> std::result::Result<GenericClaims, JWTError> {
    let jwt_audience = http_client.project_id.to_string();
    let jwt_issuer = format!("stytch.com/{}", http_client.project_id);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| JWTError::Other(Box::new(e)))?
        .as_secs();

    let kid = decode_header(jwt)
        .map_err(|e| JWTError::Other(Box::new(e)))?
        .kid
        .ok_or(JWTError::MissingField("kid".to_string()))?;
    let jwk = http_client
        .fetch_jwk(&kid)
        .await
        .map_err(|e| JWTError::Other(Box::new(e)))?;
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

    // Validate the JWT
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&jwt_audience]);
    validation.set_issuer(&[&jwt_issuer]);

    // And decode it
    let token_data = decode::<Value>(jwt, &decoding_key, &validation)?;
    let payload = token_data.claims;

    if let Some(max_age) = max_token_age_seconds {
        if let Some(iat) = payload["iat"].as_u64() {
            if now - iat >= max_age {
                return Err(JWTError::TooOld);
            }
        } else {
            return Err(JWTError::MissingField("iat".to_string()));
        }
    }

    let reserved_claim_keys = &["aud", "exp", "iat", "iss", "jti", "nbf", "sub"];
    if let Value::Object(map) = &payload {
        Ok(GenericClaims {
            reserved_claims: map
                .iter()
                .filter_map(|(k, v)| {
                    if reserved_claim_keys.contains(&k.as_str()) {
                        Some((k.clone(), v.clone()))
                    } else {
                        None
                    }
                })
                .collect(),
            untyped_claims: map
                .iter()
                .filter_map(|(k, v)| {
                    if !reserved_claim_keys.contains(&k.as_str()) {
                        Some((k.clone(), v.clone()))
                    } else {
                        None
                    }
                })
                .collect(),
        })
    } else {
        Err(JWTError::PayloadFormat)
    }
}
