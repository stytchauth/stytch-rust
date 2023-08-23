// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::consumer::sessions::AuthenticationFactor;
use crate::consumer::sessions::JWK;
use serde::{Deserialize, Serialize};

/// MemberSession:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberSession {
    /// member_session_id: Globally unique UUID that identifies a specific Session.
    pub member_session_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// started_at: The timestamp when the Session was created. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// last_accessed_at: The timestamp when the Session was last accessed. Values conform to the RFC 3339
    /// standard and are expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
    /// expires_at: The timestamp when the Session expires. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// authentication_factors: An array of different authentication factors that have initiated a Session.
    pub authentication_factors: std::vec::Vec<AuthenticationFactor>,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// custom_claims: The custom claims map for a Session. Claims can be added to a session during a Sessions
    /// authenticate call.
    pub custom_claims: std::option::Option<serde_json::Value>,
}

/// AuthenticateRequest: Request type for `Sessions.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// session_token: A secret token for a given Stytch Session.
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
    ///   If the `session_duration_minutes` parameter is not specified, a Stytch session will be created with a
    /// 60 minute duration. If you don't want
    ///   to use the Stytch session product, you can ignore the session fields in the response.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: std::option::Option<String>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in
    ///   `session_duration_minutes`. Claims will be included on the Session object and in the JWT. To update a
    /// key in an existing Session, supply a new value. To
    ///   delete a key, supply a null value. Custom claims made with reserved claims (`iss`, `sub`, `aud`,
    /// `exp`, `nbf`, `iat`, `jti`) will be ignored.
    ///   Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
}

/// AuthenticateResponse: Response type for `Sessions.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: MemberSession,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object) if one already exists, or
    /// null if one does not.
    pub member: Member,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// ExchangeRequest: Request type for `Sessions.exchange`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExchangeRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// session_token: The `session_token` belonging to the member that you wish to associate the email with.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The `session_jwt` belonging to the member that you wish to associate the email with.
    pub session_jwt: std::option::Option<String>,
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
    ///   If the `session_duration_minutes` parameter is not specified, a Stytch session will be created with a
    /// 60 minute duration. If you don't want
    ///   to use the Stytch session product, you can ignore the session fields in the response.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in
    ///   `session_duration_minutes`. Claims will be included on the Session object and in the JWT. To update a
    /// key in an existing Session, supply a new value. To
    ///   delete a key, supply a null value. Custom claims made with reserved claims (`iss`, `sub`, `aud`,
    /// `exp`, `nbf`, `iat`, `jti`) will be ignored.
    ///   Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
    /// locale: If the Member needs to complete an MFA step, and the Member has a phone number, this endpoint
    /// will pre-emptively send a one-time passcode (OTP) to the Member's phone number. The locale argument will
    /// be used to determine which language to use when sending the passcode.
    ///
    /// Parameter is a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/),
    /// e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<ExchangeRequestLocale>,
}

/// ExchangeResponse: Response type for `Sessions.exchange`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: MemberSession,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object) if one already exists, or
    /// null if one does not.
    pub member: Member,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// member_authenticated: Indicates whether the Member is fully authenticated. If false, the Member needs to
    /// complete an MFA step to log in to the Organization.
    pub member_authenticated: bool,
    /// intermediate_session_token: The returned Intermediate Session Token contains any Email Magic Link or
    /// OAuth factors from the original member session that are valid for the target Organization.
    ///       The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms) to complete the
    /// MFA flow and log in to the target Organization.
    ///       It can also be used with the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a different existing Organization,
    ///       or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization.
    pub intermediate_session_token: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
}

/// GetJWKSRequest: Request type for `Sessions.get_jwks`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetJWKSRequest {
    /// project_id: The `project_id` to get the JWKS for.
    pub project_id: String,
}

/// GetJWKSResponse: Response type for `Sessions.get_jwks`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetJWKSResponse {
    /// keys: The JWK
    pub keys: std::vec::Vec<JWK>,
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

/// GetRequest: Request type for `Sessions.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}

/// GetResponse: Response type for `Sessions.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_sessions: An array of [Session objects](https://stytch.com/docs/b2b/api/session-object).
    pub member_sessions: std::vec::Vec<MemberSession>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// RevokeRequest: Request type for `Sessions.revoke`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevokeRequest {
    /// member_session_id: Globally unique UUID that identifies a specific Session in the Stytch API. The
    /// `member_session_id` is critical to perform operations on an Session, so be sure to preserve this value.
    pub member_session_id: std::option::Option<String>,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: std::option::Option<String>,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: std::option::Option<String>,
}

/// RevokeResponse: Response type for `Sessions.revoke`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeResponse {
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ExchangeRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Sessions {
    http_client: crate::reqwest::Client,
}

impl Sessions {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let path = format!("/v1/b2b/sessions");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = format!("/v1/b2b/sessions/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn revoke(&self, body: RevokeRequest) -> crate::Result<RevokeResponse> {
        let path = format!("/v1/b2b/sessions/revoke");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn exchange(&self, body: ExchangeRequest) -> crate::Result<ExchangeResponse> {
        let path = format!("/v1/b2b/sessions/exchange");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get_jwks(&self, body: GetJWKSRequest) -> crate::Result<GetJWKSResponse> {
        let project_id = &body.project_id;
        let path = format!("/v1/b2b/sessions/jwks/{project_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}
