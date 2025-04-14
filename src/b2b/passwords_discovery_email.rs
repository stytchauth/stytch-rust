// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery::DiscoveredOrganization;
use serde::{Deserialize, Serialize};

/// ResetRequest: Request type for `Email.reset`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResetRequest {
    /// password_reset_token: The password reset token to authenticate.
    pub password_reset_token: String,
    /// password: The password to authenticate, reset, or set for the first time. Any UTF8 character is allowed,
    /// e.g. spaces, emojis, non-English characers, etc.
    pub password: String,
    pub pkce_code_verifier: std::option::Option<String>,
}
/// ResetResponse: Response type for `Email.reset`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// intermediate_session_token: The returned Intermediate Session Token contains a password factor
    /// associated with the Member. If this value is non-empty, the member must complete an MFA step to finish
    /// logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. Password factors are not transferable between Organizations, so
    /// the intermediate session token is not valid for use with discovery endpoints.
    pub intermediate_session_token: String,
    /// email_address: The email address.
    pub email_address: String,
    /// discovered_organizations: An array of `discovered_organization` objects tied to the
    /// `intermediate_session_token`, `session_token`, or `session_jwt`. See the
    /// [Discovered Organization Object](https://stytch.com/docs/b2b/api/discovered-organization-object) for
    /// complete details.
    ///
    ///   Note that Organizations will only appear here under any of the following conditions:
    ///   1. The end user is already a Member of the Organization.
    ///   2. The end user is invited to the Organization.
    ///   3. The end user can join the Organization because:
    ///
    ///   a) The Organization allows JIT provisioning.
    ///
    ///   b) The Organizations' allowed domains list contains the Member's email domain.
    ///
    ///   c) The Organization has at least one other Member with a verified email address with the same domain
    /// as the end user (to prevent phishing attacks).
    pub discovered_organizations: std::vec::Vec<DiscoveredOrganization>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// ResetStartRequest: Request type for `Email.reset_start`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResetStartRequest {
    /// email_address: The email address of the Member to start the email reset process for.
    pub email_address: String,
    /// reset_password_redirect_url: The URL that the Member clicks from the reset password link. This URL
    /// should be an endpoint in the backend server that verifies the request by querying
    ///   Stytch's authenticate endpoint and finishes the reset password flow. If this value is not passed, the
    /// default `reset_password_redirect_url` that you set in your Dashboard is used.
    ///   If you have not set a default `reset_password_redirect_url`, an error is returned.
    pub reset_password_redirect_url: std::option::Option<String>,
    /// discovery_redirect_url: The URL that the end user clicks from the discovery Magic Link. This URL should
    /// be an endpoint in the backend server that
    ///   verifies the request by querying Stytch's discovery authenticate endpoint and continues the flow. If
    /// this value is not passed, the default
    ///   discovery redirect URL that you set in your Dashboard is used. If you have not set a default discovery
    /// redirect URL, an error is returned.
    pub discovery_redirect_url: std::option::Option<String>,
    /// reset_password_template_id: Use a custom template for reset password emails. By default, it will use
    /// your default email template. The template must be a template using our built-in customizations or a
    /// custom HTML email for Passwords - Reset Password.
    pub reset_password_template_id: std::option::Option<String>,
    /// reset_password_expiration_minutes: Sets a time limit after which the email link to reset the member's
    /// password will no longer be valid.
    pub reset_password_expiration_minutes: std::option::Option<i32>,
    pub pkce_code_challenge: std::option::Option<String>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), French (`"fr"`) and Brazilian
    /// Portuguese (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<String>,
    /// verify_email_template_id: Use a custom template for verification emails sent during password reset
    /// flows. This template will be used the first time a user sets a password via a
    ///   password reset flow. By default, it will use your default email template. The template must be a
    /// template using our built-in customizations or a custom HTML email for Passwords - Email Verification.
    pub verify_email_template_id: std::option::Option<String>,
}
/// ResetStartResponse: Response type for `Email.reset_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetStartResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
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
        let path = String::from("/v1/b2b/passwords/discovery/email/reset/start");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn reset(&self, body: ResetRequest) -> crate::Result<ResetResponse> {
        let path = String::from("/v1/b2b/passwords/discovery/email/reset");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
