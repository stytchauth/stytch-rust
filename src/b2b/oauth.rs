// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::mfa::MfaRequired;
use crate::b2b::oauth_discovery::Discovery;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use crate::b2b::sessions::PrimaryRequired;
use serde::{Deserialize, Serialize};

/// ProviderValues:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderValues {
    /// scopes: The OAuth scopes included for a given provider. See each provider's section above to see which
    /// scopes are included by default and how to add custom scopes.
    pub scopes: std::vec::Vec<String>,
    /// access_token: The `access_token` that you may use to access the User's data in the provider's API.
    pub access_token: std::option::Option<String>,
    /// refresh_token: The `refresh_token` that you may use to obtain a new `access_token` for the User within
    /// the provider's API.
    pub refresh_token: std::option::Option<String>,
    pub expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// id_token: The `id_token` returned by the OAuth provider. ID Tokens are JWTs that contain structured
    /// information about a user. The exact content of each ID Token varies from provider to provider. ID Tokens
    /// are returned from OAuth providers that conform to the [OpenID Connect](https://openid.net/foundation/)
    /// specification, which is based on OAuth.
    pub id_token: std::option::Option<String>,
}
/// AuthenticateRequest: Request type for `OAuth.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// oauth_token: The token to authenticate.
    pub oauth_token: String,
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
    /// pkce_code_verifier: A base64url encoded one time secret used to validate that the request starts and
    /// ends on the same device.
    pub pkce_code_verifier: std::option::Option<String>,
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
    pub locale: std::option::Option<AuthenticateRequestLocale>,
    /// intermediate_session_token: Adds this primary authentication factor to the intermediate session token.
    /// If the resulting set of factors satisfies the organization's primary authentication requirements and MFA
    /// requirements, the intermediate session token will be consumed and converted to a member session. If not,
    /// the same intermediate session token will be returned.
    pub intermediate_session_token: std::option::Option<String>,
}
/// AuthenticateResponse: Response type for `OAuth.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the `sub` or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// provider_type: Denotes the OAuth identity provider that the user has authenticated with, e.g. Google,
    /// Microsoft, GitHub etc.
    pub provider_type: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// reset_sessions: This field is deprecated.
    pub reset_sessions: bool,
    /// member_authenticated: Indicates whether the Member is fully authenticated. If false, the Member needs to
    /// complete an MFA step to log in to the Organization.
    pub member_authenticated: bool,
    /// intermediate_session_token: The returned Intermediate Session Token contains an OAuth factor associated
    /// with the Member's email address. If this value is non-empty, the member must complete an MFA step to
    /// finish logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. The token has a default expiry of 10 minutes. It can also be
    /// used with the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member. Intermediate Session Tokens have a default expiry of 10 minutes.
    pub intermediate_session_token: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
    /// provider_values: The `provider_values` object lists relevant identifiers, values, and scopes for a given
    /// OAuth provider. For example this object will include a provider's `access_token` that you can use to
    /// access the provider's API for a given user.
    ///
    ///   Note that these values will vary based on the OAuth provider in question, e.g. `id_token` is only
    /// returned by Microsoft. Google One Tap does not return access tokens or refresh tokens.
    pub provider_values: std::option::Option<ProviderValues>,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
    /// primary_required: Information about the primary authentication requirements of the Organization.
    pub primary_required: std::option::Option<PrimaryRequired>,
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
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "deDE")]
    DeDE,
    #[serde(rename = "zhHans")]
    ZhHans,
    #[serde(rename = "caES")]
    CaES,
}

pub struct OAuth {
    http_client: crate::client::Client,
    pub discovery: Discovery,
}

impl OAuth {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            discovery: Discovery::new(http_client.clone()),
        }
    }

    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/b2b/oauth/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
