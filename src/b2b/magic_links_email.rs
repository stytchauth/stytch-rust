// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::magic_links_email_discovery::Discovery;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use serde::{Deserialize, Serialize};

/// InviteRequest: Request type for `Email.invite`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InviteRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// email_address: The email address of the Member.
    pub email_address: String,
    /// invite_redirect_url: The URL that the Member clicks from the invite Email Magic Link. This URL should be
    /// an endpoint in the backend server that verifies
    ///   the request by querying Stytch's authenticate endpoint and finishes the invite flow. If this value is
    /// not passed, the default `invite_redirect_url`
    ///   that you set in your Dashboard is used. If you have not set a default `invite_redirect_url`, an error
    /// is returned.
    pub invite_redirect_url: std::option::Option<String>,
    /// invited_by_member_id: The `member_id` of the Member who sends the invite.
    pub invited_by_member_id: std::option::Option<String>,
    /// name: The name of the Member.
    pub name: std::option::Option<String>,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data or
    /// identity-provider-specific data.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: An arbitrary JSON object of application-specific data. These fields can be edited
    /// directly by the
    ///   frontend SDK, and should not be used to store critical information. See the
    /// [Metadata resource](https://stytch.com/docs/b2b/api/metadata)
    ///   for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
    /// invite_template_id: Use a custom template for invite emails. By default, it will use your default email
    /// template. The template must be a template
    ///   using our built-in customizations or a custom HTML email for Magic Links - Invite.
    pub invite_template_id: std::option::Option<String>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<InviteRequestLocale>,
    /// roles: Roles to explicitly assign to this Member. See the
    /// [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment)
    ///    for more information about role assignment.
    pub roles: std::option::Option<std::vec::Vec<String>>,
}

/// InviteResponse: Response type for `Email.invite`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteResponse {
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

/// LoginOrSignupRequest: Request type for `Email.login_or_signup`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginOrSignupRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// email_address: The email address of the Member.
    pub email_address: String,
    /// login_redirect_url: The URL that the Member clicks from the login Email Magic Link. This URL should be
    /// an endpoint in the backend server that
    ///   verifies the request by querying Stytch's authenticate endpoint and finishes the login. If this value
    /// is not passed, the default login
    ///   redirect URL that you set in your Dashboard is used. If you have not set a default login redirect URL,
    /// an error is returned.
    pub login_redirect_url: std::option::Option<String>,
    /// signup_redirect_url: The URL the Member clicks from the signup Email Magic Link. This URL should be an
    /// endpoint in the backend server that verifies
    ///   the request by querying Stytch's authenticate endpoint and finishes the login. If this value is not
    /// passed, the default sign-up redirect URL
    ///   that you set in your Dashboard is used. If you have not set a default sign-up redirect URL, an error
    /// is returned.
    pub signup_redirect_url: std::option::Option<String>,
    /// pkce_code_challenge: A base64url encoded SHA256 hash of a one time secret used to validate that the
    /// request starts and ends on the same device.
    pub pkce_code_challenge: std::option::Option<String>,
    /// login_template_id: Use a custom template for login emails. By default, it will use your default email
    /// template. The template must be from Stytch's
    /// built-in customizations or a custom HTML email for Magic Links - Login.
    pub login_template_id: std::option::Option<String>,
    /// signup_template_id: Use a custom template for signup emails. By default, it will use your default email
    /// template. The template must be from Stytch's
    /// built-in customizations or a custom HTML email for Magic Links - Signup.
    pub signup_template_id: std::option::Option<String>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<LoginOrSignupRequestLocale>,
}

/// LoginOrSignupResponse: Response type for `Email.login_or_signup`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginOrSignupResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// member_created: A flag indicating `true` if a new Member object was created and `false` if the Member
    /// object already existed.
    pub member_created: bool,
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
pub enum InviteRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum LoginOrSignupRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Email {
    http_client: crate::client::Client,
    pub discovery: Discovery,
}

impl Email {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            discovery: Discovery::new(http_client.clone()),
        }
    }

    pub async fn login_or_signup(
        &self,
        body: LoginOrSignupRequest,
    ) -> crate::Result<LoginOrSignupResponse> {
        let path = String::from("/v1/b2b/magic_links/email/login_or_signup");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn invite(&self, body: InviteRequest) -> crate::Result<InviteResponse> {
        let path = String::from("/v1/b2b/magic_links/email/invite");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
