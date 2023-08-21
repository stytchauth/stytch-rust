// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::sessions::Session;
use crate::consumer::users::User;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `CryptoWallets.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateRequest {
    /// crypto_wallet_type: The type of wallet to authenticate. Currently `ethereum` and `solana` are supported.
    /// Wallets for any EVM-compatible chains (such as Polygon or BSC) are also supported and are grouped under
    /// the `ethereum` type.
    pub crypto_wallet_type: String,
    /// crypto_wallet_address: The crypto wallet address to authenticate.
    pub crypto_wallet_address: String,
    /// signature: The signature from the message challenge.
    pub signature: String,
    /// session_token: The `session_token` associated with a User's existing Session.
    pub session_token: std::option::Option<String>,
    /// session_duration_minutes: Set the session lifetime to be this many minutes from now. This will start a
    /// new session if one doesn't already exist,
    ///   returning both an opaque `session_token` and `session_jwt` for this session. Remember that the
    /// `session_jwt` will have a fixed lifetime of
    ///   five minutes regardless of the underlying session duration, and will need to be refreshed over time.
    ///
    ///   This value must be a minimum of 5 and a maximum of 527040 minutes (366 days).
    ///   
    ///   If a `session_token` or `session_jwt` is provided then a successful authentication will continue to
    /// extend the session this many minutes.
    ///   
    ///   If the `session_duration_minutes` parameter is not specified, a Stytch session will not be created.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_jwt: The `session_jwt` associated with a User's existing Session.
    pub session_jwt: std::option::Option<String>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in `session_duration_minutes`. Claims will be
    /// included on the Session object and in the JWT. To update a key in an existing Session, supply a new
    /// value. To delete a key, supply a null value.
    ///
    ///   Custom claims made with reserved claims ("iss", "sub", "aud", "exp", "nbf", "iat", "jti") will be
    /// ignored. Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<String>,
}

/// AuthenticateResponse: Response type for `CryptoWallets.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// session: If you initiate a Session, by including `session_duration_minutes` in your authenticate call,
    /// you'll receive a full Session object in the response.
    ///
    ///   See [GET sessions](https://stytch.com/docs/api/session-get) for complete response fields.
    ///   
    pub session: std::option::Option<Session>,
}

/// AuthenticateStartRequest: Request type for `CryptoWallets.authenticate_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateStartRequest {
    /// crypto_wallet_type: The type of wallet to authenticate. Currently `ethereum` and `solana` are supported.
    /// Wallets for any EVM-compatible chains (such as Polygon or BSC) are also supported and are grouped under
    /// the `ethereum` type.
    pub crypto_wallet_type: String,
    /// crypto_wallet_address: The crypto wallet address to authenticate.
    pub crypto_wallet_address: String,
    /// user_id: The unique ID of a specific User.
    pub user_id: std::option::Option<String>,
    /// session_token: The `session_token` associated with a User's existing Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The `session_jwt` associated with a User's existing Session.
    pub session_jwt: std::option::Option<String>,
}

/// AuthenticateStartResponse: Response type for `CryptoWallets.authenticate_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateStartResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// challenge: A challenge string to be signed by the wallet in order to prove ownership.
    pub challenge: String,
    /// user_created: In `login_or_create` endpoints, this field indicates whether or not a User was just
    /// created.
    pub user_created: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

pub struct CryptoWallets {
    http_client: crate::reqwest::Client,
}

impl CryptoWallets {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn authenticate_start(
        &self,
        body: AuthenticateStartRequest,
    ) -> crate::Result<AuthenticateStartResponse> {
        let path = format!("/v1/crypto_wallets/authenticate/start");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = format!("/v1/crypto_wallets/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}