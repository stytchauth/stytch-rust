// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use serde::{Deserialize, Serialize};

/// ResetRequest: Request type for `Sessions.reset`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResetRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// password: The password to authenticate.
    pub password: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: std::option::Option<String>,
    pub session_duration_minutes: std::option::Option<i32>,
    pub session_custom_claims: std::option::Option<serde_json::Value>,
    pub locale: std::option::Option<ResetRequestLocale>,
}

/// ResetResponse: Response type for `Sessions.reset`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    pub session_token: String,
    pub session_jwt: String,
    pub intermediate_session_token: String,
    pub member_authenticated: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
    pub mfa_required: std::option::Option<MfaRequired>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ResetRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Sessions {
    http_client: crate::client::Client,
}

impl Sessions {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn reset(&self, body: ResetRequest) -> crate::Result<ResetResponse> {
        let path = String::from("/v1/b2b/passwords/session/reset");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
