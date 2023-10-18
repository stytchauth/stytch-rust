// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use crate::consumer::magic_links::Options;
use crate::consumer::otp_email::Email;
use crate::consumer::otp_sms::Sms;
use crate::consumer::otp_whatsapp::Whatsapp;
use crate::consumer::sessions::Session;
use crate::consumer::users::User;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `OTPs.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// method_id: The `email_id` or `phone_id` involved in the given authentication.
    pub method_id: String,
    /// code: The code to authenticate.
    pub code: String,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// options: Specify optional security settings.
    pub options: std::option::Option<Options>,
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
    pub session_custom_claims: std::option::Option<serde_json::Value>,
}

/// AuthenticateResponse: Response type for `OTPs.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// method_id: The `email_id` or `phone_id` involved in the given authentication.
    pub method_id: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// reset_sessions: Indicates if all other of the User's Sessions need to be reset. You should check this
    /// field if you aren't using Stytch's Session product. If you are using Stytch's Session product, we revoke
    /// the User's other sessions for you.
    pub reset_sessions: bool,
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

pub struct OTPs {
    http_client: crate::client::Client,
    pub sms: Sms,
    pub whatsapp: Whatsapp,
    pub email: Email,
}

impl OTPs {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            sms: Sms::new(http_client.clone()),
            whatsapp: Whatsapp::new(http_client.clone()),
            email: Email::new(http_client.clone()),
        }
    }

    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/otps/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
