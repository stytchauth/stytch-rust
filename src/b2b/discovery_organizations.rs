// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery::DiscoveredOrganization;
use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::EmailImplicitRoleAssignment;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::sessions::MemberSession;
use crate::b2b::sessions::PrimaryRequired;
use serde::{Deserialize, Serialize};

/// CreateRequest: Request type for `Organizations.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// intermediate_session_token: The Intermediate Session Token. This token does not necessarily belong to a
    /// specific instance of a Member, but represents a bag of factors that may be converted to a member
    /// session. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. It can also be used with the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member.
    pub intermediate_session_token: String,
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
    /// organization_name: The name of the Organization. If the name is not specified, a default name will be
    /// created based on the email used to initiate the discovery flow. If the email domain is a common email
    /// provider such as gmail.com, or if the email is a .edu email, the organization name will be generated
    /// based on the name portion of the email. Otherwise, the organization name will be generated based on the
    /// email domain.
    pub organization_name: std::option::Option<String>,
    /// organization_slug: The unique URL slug of the Organization. A minimum of two characters is required. The
    /// slug only accepts alphanumeric characters and the following reserved characters: `-` `.` `_` `~`. If the
    /// slug is not specified, a default slug will be created based on the email used to initiate the discovery
    /// flow. If the email domain is a common email provider such as gmail.com, or if the email is a .edu email,
    /// the organization slug will be generated based on the name portion of the email. Otherwise, the
    /// organization slug will be generated based on the email domain.
    pub organization_slug: std::option::Option<String>,
    /// organization_logo_url: The image URL of the Organization logo.
    pub organization_logo_url: std::option::Option<String>,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data or
    /// identity-provider-specific data.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// sso_jit_provisioning: The authentication setting that controls the JIT provisioning of Members when
    /// authenticating via SSO. The accepted values are:
    ///
    ///   `ALL_ALLOWED` – new Members will be automatically provisioned upon successful authentication via any
    /// of the Organization's `sso_active_connections`.
    ///
    ///   `RESTRICTED` – only new Members with SSO logins that comply with
    /// `sso_jit_provisioning_allowed_connections` can be provisioned upon authentication.
    ///
    ///   `NOT_ALLOWED` – disable JIT provisioning via SSO.
    ///
    pub sso_jit_provisioning: std::option::Option<String>,
    /// email_allowed_domains: An array of email domains that allow invites or JIT provisioning for new Members.
    /// This list is enforced when either `email_invites` or `email_jit_provisioning` is set to `RESTRICTED`.
    ///
    ///
    /// Common domains such as `gmail.com` are not allowed. See the
    /// [common email domains resource](https://stytch.com/docs/b2b/api/common-email-domains) for the full list.
    pub email_allowed_domains: std::option::Option<std::vec::Vec<String>>,
    /// email_jit_provisioning: The authentication setting that controls how a new Member can be provisioned by
    /// authenticating via Email Magic Link or OAuth. The accepted values are:
    ///
    ///   `RESTRICTED` – only new Members with verified emails that comply with `email_allowed_domains` can be
    /// provisioned upon authentication via Email Magic Link or OAuth.
    ///
    ///   `NOT_ALLOWED` – disable JIT provisioning via Email Magic Link and OAuth.
    ///
    pub email_jit_provisioning: std::option::Option<String>,
    /// email_invites: The authentication setting that controls how a new Member can be invited to an
    /// organization by email. The accepted values are:
    ///
    ///   `ALL_ALLOWED` – any new Member can be invited to join via email.
    ///
    ///   `RESTRICTED` – only new Members with verified emails that comply with `email_allowed_domains` can be
    /// invited via email.
    ///
    ///   `NOT_ALLOWED` – disable email invites.
    ///
    pub email_invites: std::option::Option<String>,
    /// auth_methods: The setting that controls which authentication methods can be used by Members of an
    /// Organization. The accepted values are:
    ///
    ///   `ALL_ALLOWED` – the default setting which allows all authentication methods to be used.
    ///
    ///   `RESTRICTED` – only methods that comply with `allowed_auth_methods` can be used for authentication.
    /// This setting does not apply to Members with `is_breakglass` set to `true`.
    ///
    pub auth_methods: std::option::Option<String>,
    /// allowed_auth_methods: An array of allowed authentication methods. This list is enforced when
    /// `auth_methods` is set to `RESTRICTED`.
    ///   The list's accepted values are: `sso`, `magic_link`, `email_otp`, `password`, `google_oauth`,
    /// `microsoft_oauth`, `slack_oauth`, `github_oauth`, and `hubspot_oauth`.
    ///
    pub allowed_auth_methods: std::option::Option<std::vec::Vec<String>>,
    /// mfa_policy: The setting that controls the MFA policy for all Members in the Organization. The accepted
    /// values are:
    ///
    ///   `REQUIRED_FOR_ALL` – All Members within the Organization will be required to complete MFA every time
    /// they wish to log in. However, any active Session that existed prior to this setting change will remain
    /// valid.
    ///
    ///   `OPTIONAL` – The default value. The Organization does not require MFA by default for all Members.
    /// Members will be required to complete MFA only if their `mfa_enrolled` status is set to true.
    ///
    pub mfa_policy: std::option::Option<String>,
    /// rbac_email_implicit_role_assignments: Implicit role assignments based off of email domains.
    ///   For each domain-Role pair, all Members whose email addresses have the specified email domain will be
    /// granted the
    ///   associated Role, regardless of their login method. See the
    /// [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment)
    ///   for more information about role assignment.
    pub rbac_email_implicit_role_assignments:
        std::option::Option<std::vec::Vec<EmailImplicitRoleAssignment>>,
    /// mfa_methods: The setting that controls which MFA methods can be used by Members of an Organization. The
    /// accepted values are:
    ///
    ///   `ALL_ALLOWED` – the default setting which allows all authentication methods to be used.
    ///
    ///   `RESTRICTED` – only methods that comply with `allowed_mfa_methods` can be used for authentication.
    /// This setting does not apply to Members with `is_breakglass` set to `true`.
    ///
    pub mfa_methods: std::option::Option<String>,
    /// allowed_mfa_methods: An array of allowed MFA authentication methods. This list is enforced when
    /// `mfa_methods` is set to `RESTRICTED`.
    ///   The list's accepted values are: `sms_otp` and `totp`.
    ///
    pub allowed_mfa_methods: std::option::Option<std::vec::Vec<String>>,
    /// oauth_tenant_jit_provisioning: The authentication setting that controls how a new Member can JIT
    /// provision into an organization by tenant. The accepted values are:
    ///
    ///   `RESTRICTED` – only new Members with tenants in `allowed_oauth_tenants` can JIT provision via tenant.
    ///
    ///   `NOT_ALLOWED` – disable JIT provisioning by OAuth Tenant.
    ///
    pub oauth_tenant_jit_provisioning: std::option::Option<String>,
    /// allowed_oauth_tenants: A map of allowed OAuth tenants. If this field is not passed in, the Organization
    /// will not allow JIT provisioning by OAuth Tenant. Allowed keys are "slack", "hubspot", and "github".
    pub allowed_oauth_tenants: std::option::Option<serde_json::Value>,
}
/// CreateResponse: Response type for `Organizations.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// member_authenticated: Indicates whether the Member is fully authenticated. If false, the Member needs to
    /// complete an MFA step to log in to the Organization.
    pub member_authenticated: bool,
    /// intermediate_session_token: The returned Intermediate Session Token is identical to the one that was
    /// originally passed in to the request. If this value is non-empty, the member must complete an MFA step to
    /// finish logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. It can also be used with the
    /// [Exchange Intermediate Session endpoint](https://stytch.com/docs/b2b/api/exchange-intermediate-session)
    /// to join a specific Organization that allows the factors represented by the intermediate session token;
    /// or the
    /// [Create Organization via Discovery endpoint](https://stytch.com/docs/b2b/api/create-organization-via-discovery) to create a new Organization and Member.
    pub intermediate_session_token: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: std::option::Option<Organization>,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
    /// primary_required: Information about the primary authentication requirements of the Organization.
    pub primary_required: std::option::Option<PrimaryRequired>,
}
/// ListRequest: Request type for `Organizations.list`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ListRequest {
    /// intermediate_session_token: The Intermediate Session Token. This token does not necessarily belong to a
    /// specific instance of a Member, but represents a bag of factors that may be converted to a member
    /// session. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. It can also be used with the
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
/// ListResponse: Response type for `Organizations.list`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
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
    /// organization_id_hint: If the intermediate session token is associated with a specific Organization, that
    /// Organization ID will be returned here. The Organization ID will be null if the intermediate session
    /// token was generated by a email magic link discovery or OAuth discovery flow. If a session token or
    /// session JWT is provided, the Organization ID hint will be null.
    pub organization_id_hint: std::option::Option<String>,
}

pub struct Organizations {
    http_client: crate::client::Client,
}

impl Organizations {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let path = String::from("/v1/b2b/discovery/organizations/create");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn list(&self, body: ListRequest) -> crate::Result<ListResponse> {
        let path = String::from("/v1/b2b/discovery/organizations");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
