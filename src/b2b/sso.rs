// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use crate::b2b::sso_oidc::OIDC;
use crate::b2b::sso_saml::SAML;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OIDCConnection {
    pub organization_id: String,
    pub connection_id: String,
    pub status: String,
    pub display_name: String,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub issuer: String,
    pub authorization_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub jwks_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SAMLConnection {
    pub organization_id: String,
    pub connection_id: String,
    pub status: String,
    pub idp_entity_id: String,
    pub display_name: String,
    pub idp_sso_url: String,
    pub acs_url: String,
    pub audience_uri: String,
    pub signing_certificates: std::vec::Vec<X509Certificate>,
    pub verification_certificates: std::vec::Vec<X509Certificate>,
    pub attribute_mapping: std::option::Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct X509Certificate {
    pub certificate_id: String,
    pub certificate: String,
    pub issuer: String,
    pub created_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
}

/// AuthenticateRequest: Request type for `SSO.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateRequest {
    /// sso_token: The token to authenticate.
    pub sso_token: String,
    /// pkce_code_verifier: A base64url encoded one time secret used to validate that the request starts and
    /// ends on the same device.
    pub pkce_code_verifier: std::option::Option<String>,
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
    pub session_custom_claims: std::option::Option<String>,
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
}

/// AuthenticateResponse: Response type for `SSO.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object) if one already exists, or
    /// null if one does not.
    pub member: Member,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// reset_session: Indicates if all Sessions linked to the Member need to be reset. You should check this
    /// field if you aren't using
    ///     Stytch's Session product. If you are using Stytch's Session product, we revoke the Member’s other
    /// Sessions for you.
    pub reset_session: bool,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// intermediate_session_token: The returned Intermediate Session Token contains an SSO factor associated
    /// with the Member.
    ///       The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms) to complete the
    /// MFA flow and log in to the Organization.
    ///       SSO factors are not transferable between Organizations, so the intermediate session token is not
    /// valid for use with discovery endpoints.
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

/// DeleteConnectionRequest: Request type for `SSO.delete_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteConnectionRequest {
    /// organization_id: The organization ID that the SSO connection belongs to.
    pub organization_id: String,
    /// connection_id: The ID of the SSO connection. Both SAML and OIDC connection IDs can be provided.
    pub connection_id: String,
}

/// DeleteConnectionResponse: Response type for `SSO.delete_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteConnectionResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// connection_id: The `connection_id` that was deleted as part of the delete request.
    pub connection_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// GetConnectionsRequest: Request type for `SSO.get_connections`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetConnectionsRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
}

/// GetConnectionsResponse: Response type for `SSO.get_connections`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetConnectionsResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// saml_connections: The list of [SAML Connections](https://stytch.com/docs/b2b/api/saml-connection-object)
    /// owned by this organization.
    pub saml_connections: std::vec::Vec<SAMLConnection>,
    /// oidc_connections: The list of [OIDC Connections](https://stytch.com/docs/b2b/api/oidc-connection-object)
    /// owned by this organization.
    pub oidc_connections: std::vec::Vec<OIDCConnection>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthenticateRequestLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct SSO {
    http_client: crate::reqwest::Client,
    pub oidc: OIDC,
    pub saml: SAML,
}

impl SSO {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            oidc: OIDC::new(http_client.clone()),
            saml: SAML::new(http_client.clone()),
        }
    }

    pub async fn get_connections(
        &self,
        body: GetConnectionsRequest,
    ) -> crate::Result<GetConnectionsResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/sso/{organization_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn delete_connection(
        &self,
        body: DeleteConnectionRequest,
    ) -> crate::Result<DeleteConnectionResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/sso/{organization_id}/connections/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = format!("/v1/b2b/sso/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}