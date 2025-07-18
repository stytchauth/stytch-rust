// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use crate::consumer::magic_links::Options;
use crate::consumer::sessions::Session;
use crate::consumer::users::User;
use serde::{Deserialize, Serialize};

/// ResetRequest: Request type for `Email.reset`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResetRequest {
    /// token: The Passwords `token` from the `?token=` query parameter in the URL.
    ///
    ///   In the redirect URL, the `stytch_token_type` will be `login` or `reset_password`.
    ///
    ///   See examples and read more about redirect URLs
    /// [here](https://stytch.com/docs/workspace-management/redirect-urls).
    pub token: String,
    /// password: The password for the user. Any UTF8 character is allowed, e.g. spaces, emojis, non-English
    /// characters, etc.
    pub password: String,
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
    /// code_verifier: A base64url encoded one time secret used to validate that the request starts and ends on
    /// the same device.
    pub code_verifier: std::option::Option<String>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in `session_duration_minutes`. Claims will be
    /// included on the Session object and in the JWT. To update a key in an existing Session, supply a new
    /// value. To delete a key, supply a null value.
    ///
    ///   Custom claims made with reserved claims ("iss", "sub", "aud", "exp", "nbf", "iat", "jti") will be
    /// ignored. Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
    /// attributes: Provided attributes to help with fraud detection. These values are pulled and passed into
    /// Stytch endpoints by your application.
    pub attributes: std::option::Option<Attributes>,
    /// options: Specify optional security settings.
    pub options: std::option::Option<Options>,
}
/// ResetResponse: Response type for `Email.reset`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetResponse {
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
    ///   See [Session object](https://stytch.com/docs/api/session-object) for complete response fields.
    ///
    pub session: std::option::Option<Session>,
}
/// ResetStartRequest: Request type for `Email.reset_start`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResetStartRequest {
    /// email: The email of the User that requested the password reset.
    pub email: String,
    /// reset_password_redirect_url: The url that the user clicks from the password reset email to finish the
    /// reset password flow.
    ///   This should be a url that your app receives and parses before showing your app's reset password page.
    ///   After the user submits a new password to your app, it should send an API request to complete the
    /// password reset process.
    ///   If this value is not passed, the default reset password redirect URL that you set in your Dashboard is
    /// used.
    ///   If you have not set a default reset password redirect URL, an error is returned.
    pub reset_password_redirect_url: std::option::Option<String>,
    /// reset_password_expiration_minutes: Set the expiration for the password reset, in minutes. By default, it
    /// expires in 30 minutes.
    ///   The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub reset_password_expiration_minutes: std::option::Option<i32>,
    /// code_challenge: A base64url encoded SHA256 hash of a one time secret used to validate that the request
    /// starts and ends on the same device.
    pub code_challenge: std::option::Option<String>,
    /// attributes: Provided attributes to help with fraud detection. These values are pulled and passed into
    /// Stytch endpoints by your application.
    pub attributes: std::option::Option<Attributes>,
    /// login_redirect_url: The URL Stytch redirects to after the OAuth flow is completed for a user that
    /// already exists. This URL should be a route in your application which will run `oauth.authenticate` (see
    /// below) and finish the login.
    ///
    ///   The URL must be configured as a Login URL in the
    /// [Redirect URL page](https://stytch.com/docs/dashboard/redirect-urls). If the field is not specified, the
    /// default Login URL will be used.
    pub login_redirect_url: std::option::Option<String>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), French (`"fr"`) and Brazilian
    /// Portuguese (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<ResetStartRequestLocale>,
    /// reset_password_template_id: Use a custom template for password reset emails. By default, it will use
    /// your default email template.
    ///   The template must be a template using our built-in customizations or a custom HTML email for Passwords
    /// - Password reset.
    pub reset_password_template_id: std::option::Option<String>,
}
/// ResetStartResponse: Response type for `Email.reset_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetStartResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// email_id: The unique ID of a specific email address.
    pub email_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ResetStartRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
    #[serde(rename = "fr")]
    Fr,
}

pub struct Email {
    http_client: crate::client::Client,
}

impl Email {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn reset_start(&self, body: ResetStartRequest) -> crate::Result<ResetStartResponse> {
        let path = String::from("/v1/passwords/email/reset/start");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn reset(&self, body: ResetRequest) -> crate::Result<ResetResponse> {
        let path = String::from("/v1/passwords/email/reset");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
