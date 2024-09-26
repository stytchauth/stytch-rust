// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::magic_links_discovery::Discovery;
use crate::b2b::magic_links_email::Email;
use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `MagicLinks.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// magic_links_token: The Email Magic Link token to authenticate.
    pub magic_links_token: String,
    /// pkce_code_verifier: A base64url encoded one time secret used to validate that the request starts and
    /// ends on the same device.
    pub pkce_code_verifier: std::option::Option<String>,
    /// session_token: Reuse an existing session instead of creating a new one. If you provide a
    /// `session_token`, Stytch will update the session.
    ///   If the `session_token` and `magic_links_token` belong to different Members, the `session_token` will
    /// be ignored. This endpoint will error if
    ///   both `session_token` and `session_jwt` are provided.
    pub session_token: std::option::Option<String>,
    /// session_jwt: Reuse an existing session instead of creating a new one. If you provide a `session_jwt`,
    /// Stytch will update the session. If the `session_jwt`
    ///   and `magic_links_token` belong to different Members, the `session_jwt` will be ignored. This endpoint
    /// will error if both `session_token` and `session_jwt`
    ///   are provided.
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
    /// locale: If the needs to complete an MFA step, and the Member has a phone number, this endpoint will
    /// pre-emptively send a one-time passcode (OTP) to the Member's phone number. The locale argument will be
    /// used to determine which language to use when sending the passcode.
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
    pub locale: std::option::Option<AuthenticateRequestLocale>,
    /// intermediate_session_token: Adds this primary authentication factor to the intermediate session token.
    /// If the resulting set of factors satisfies the organization's primary authentication requirements and MFA
    /// requirements, the intermediate session token will be consumed and converted to a member session. If not,
    /// the same intermediate session token will be returned.
    pub intermediate_session_token: std::option::Option<String>,
}
/// AuthenticateResponse: Response type for `MagicLinks.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// method_id: The email or device involved in the authentication.
    pub method_id: String,
    /// reset_sessions: Indicates if all Sessions linked to the Member need to be reset. You should check this
    /// field if you aren't using
    /// Stytch's Session product. If you are using Stytch's Session product, we revoke the Member’s other
    /// Sessions for you.
    pub reset_sessions: bool,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// intermediate_session_token: The returned Intermediate Session Token contains an Email Magic Link factor
    /// associated with the Member's email address. If this value is non-empty, the member must complete an MFA
    /// step to finish logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. It can also be used with the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member.
    pub intermediate_session_token: String,
    /// member_authenticated: Indicates whether the Member is fully authenticated. If false, the Member needs to
    /// complete an MFA step to log in to the Organization.
    pub member_authenticated: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AuthenticateRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct MagicLinks {
    http_client: crate::client::Client,
    pub email: Email,
    pub discovery: Discovery,
}

impl MagicLinks {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            email: Email::new(http_client.clone()),
            discovery: Discovery::new(http_client.clone()),
        }
    }

    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/b2b/magic_links/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
