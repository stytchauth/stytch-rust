// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `Sms.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
    /// code: The code to authenticate.
    pub code: String,
    /// intermediate_session_token: The Intermediate Session Token. This token does not necessarily belong to a
    /// specific instance of a Member, but represents a bag of factors that may be converted to a member session.
    /// The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms) to complete an MFA
    /// flow;
    /// the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member.
    pub intermediate_session_token: std::option::Option<String>,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
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
    /// set_mfa_enrollment: Optionally sets the Member’s MFA enrollment status upon a successful authentication.
    /// If the Organization’s MFA policy is `REQUIRED_FOR_ALL`, this field will be ignored. If this field is not
    /// passed in, the Member’s `mfa_enrolled` boolean will not be affected. The options are:
    ///
    ///   `enroll` – sets the Member's `mfa_enrolled` boolean to `true`. The Member will be required to complete
    /// an MFA step upon subsequent logins to the Organization.
    ///
    ///   `unenroll` –  sets the Member's `mfa_enrolled` boolean to `false`. The Member will no longer be
    /// required to complete MFA steps when logging in to the Organization.
    ///
    pub set_mfa_enrollment: std::option::Option<String>,
    pub set_default_mfa: std::option::Option<bool>,
}

/// AuthenticateResponse: Response type for `Sms.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
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
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
}

/// SendRequest: Request type for `Sms.send`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SendRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
    /// mfa_phone_number: The phone number to send the OTP to. If the Member already has a phone number, this
    /// argument is not needed.
    pub mfa_phone_number: std::option::Option<String>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<SendRequestLocale>,
    /// intermediate_session_token: The Intermediate Session Token. This token does not necessarily belong to a
    /// specific instance of a Member, but represents a bag of factors that may be converted to a member session.
    /// The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms) to complete an MFA
    /// flow;
    /// the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member.
    pub intermediate_session_token: std::option::Option<String>,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: std::option::Option<String>,
}

/// SendResponse: Response type for `Sms.send`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendResponse {
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
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum SendRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Sms {
    http_client: crate::client::Client,
}

impl Sms {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn send(&self, body: SendRequest) -> crate::Result<SendResponse> {
        let path = String::from("/v1/b2b/otps/sms/send");
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
        let path = String::from("/v1/b2b/otps/sms/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
