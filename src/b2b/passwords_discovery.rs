// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery::DiscoveredOrganization;
use crate::b2b::passwords_discovery_email::Email;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `Discovery.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// email_address: The email address of the Member.
    pub email_address: String,
    /// password: The password to authenticate, reset, or set for the first time. Any UTF8 character is allowed,
    /// e.g. spaces, emojis, non-English characters, etc.
    pub password: String,
}
/// AuthenticateResponse: Response type for `Discovery.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// email_address: The email address.
    pub email_address: String,
    /// intermediate_session_token: The returned Intermediate Session Token contains a password factor
    /// associated with the Member. If this value is non-empty, the member must complete an MFA step to finish
    /// logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. The token has a default expiry of 10 minutes. Password factors
    /// are not transferable between Organizations, so the intermediate session token is not valid for use with
    /// discovery endpoints.
    pub intermediate_session_token: String,
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

pub struct Discovery {
    http_client: crate::client::Client,
    pub email: Email,
}

impl Discovery {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            email: Email::new(http_client.clone()),
        }
    }

    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/b2b/passwords/discovery/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
