// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::sessions::Session;
use crate::consumer::users::User;
use serde::{Deserialize, Serialize};

/// TOTP:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOTP {
    /// totp_id: The unique ID for a TOTP instance.
    pub totp_id: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
    /// recovery_codes: The recovery codes used to authenticate the user without an authenticator app.
    pub recovery_codes: std::vec::Vec<String>,
}

/// AuthenticateRequest: Request type for `TOTPs.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateRequest {
    /// user_id: The `user_id` of an active user the TOTP registration should be tied to.
    pub user_id: String,
    /// totp_code: The TOTP code to authenticate. The TOTP code should consist of 6 digits.
    pub totp_code: String,
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

/// AuthenticateResponse: Response type for `TOTPs.authenticate`.
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
    /// totp_id: The unique ID for a TOTP instance.
    pub totp_id: String,
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

/// CreateRequest: Request type for `TOTPs.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRequest {
    /// user_id: The `user_id` of an active user the TOTP registration should be tied to.
    pub user_id: String,
    /// expiration_minutes: The expiration for the TOTP instance. If the newly created TOTP is not authenticated
    /// within this time frame the TOTP will be unusable. Defaults to 60 (1 hour) with a minimum of 5 and a
    /// maximum of 1440.
    pub expiration_minutes: std::option::Option<i32>,
}

/// CreateResponse: Response type for `TOTPs.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// totp_id: The unique ID for a TOTP instance.
    pub totp_id: String,
    /// secret: The TOTP secret key shared between the authenticator app and the server used to generate TOTP
    /// codes.
    pub secret: String,
    /// qr_code: The QR code image encoded in base64.
    pub qr_code: String,
    /// recovery_codes: The recovery codes used to authenticate the user without an authenticator app.
    pub recovery_codes: std::vec::Vec<String>,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// RecoverRequest: Request type for `TOTPs.recover`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverRequest {
    /// user_id: The `user_id` of an active user the TOTP registration should be tied to.
    pub user_id: String,
    /// recovery_code: The recovery code to authenticate.
    pub recovery_code: String,
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

/// RecoverResponse: Response type for `TOTPs.recover`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// totp_id: The unique ID for a TOTP instance.
    pub totp_id: String,
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

/// RecoveryCodesRequest: Request type for `TOTPs.recovery_codes`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryCodesRequest {
    /// user_id: The `user_id` of an active user the TOTP registration should be tied to.
    pub user_id: String,
}

/// RecoveryCodesResponse: Response type for `TOTPs.recovery_codes`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryCodesResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// totps: An array containing a list of all TOTP instances (along with their recovery codes) for a given
    /// User in the Stytch API.
    pub totps: std::vec::Vec<TOTP>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

pub struct TOTPs {
    http_client: crate::reqwest::Client,
}

impl TOTPs {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let path = format!("/v1/totps");
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
        let path = format!("/v1/totps/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn recovery_codes(
        &self,
        body: RecoveryCodesRequest,
    ) -> crate::Result<RecoveryCodesResponse> {
        let path = format!("/v1/totps/recovery_codes");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn recover(&self, body: RecoverRequest) -> crate::Result<RecoverResponse> {
        let path = format!("/v1/totps/recover");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}