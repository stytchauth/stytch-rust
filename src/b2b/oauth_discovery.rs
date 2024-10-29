// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery::DiscoveredOrganization;
use serde::{Deserialize, Serialize};

/// AuthenticateRequest: Request type for `Discovery.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// discovery_oauth_token: The Discovery OAuth token to authenticate.
    pub discovery_oauth_token: String,
    pub session_token: std::option::Option<String>,
    pub session_duration_minutes: std::option::Option<i32>,
    pub session_jwt: std::option::Option<String>,
    pub session_custom_claims: std::option::Option<serde_json::Value>,
    /// pkce_code_verifier: A base64url encoded one time secret used to validate that the request starts and
    /// ends on the same device.
    pub pkce_code_verifier: std::option::Option<String>,
}
/// AuthenticateResponse: Response type for `Discovery.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
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
    /// provider_type: Denotes the OAuth identity provider that the user has authenticated with, e.g. Google,
    /// Microsoft, GitHub etc.
    pub provider_type: String,
    /// provider_tenant_id: The tenant ID returned by the OAuth provider. This is typically used to identify an
    /// organization or group within the provider's domain. For example, in HubSpot this is a Hub ID, in Slack
    /// this is the Workspace ID, and in GitHub this is an organization ID. This field will only be populated if
    /// exactly one tenant ID is returned from a successful OAuth authentication and developers should prefer
    /// `provider_tenant_ids` over this since it accounts for the possibility of an OAuth provider yielding
    /// multiple tenant IDs.
    pub provider_tenant_id: String,
    /// provider_tenant_ids: All tenant IDs returned by the OAuth provider. These is typically used to identify
    /// organizations or groups within the provider's domain. For example, in HubSpot this is a Hub ID, in Slack
    /// this is the Workspace ID, and in GitHub this is an organization ID. Some OAuth providers do not return
    /// tenant IDs, some providers are guaranteed to return one, and some may return multiple. This field will
    /// always be populated if at least one tenant ID was returned from the OAuth provider and developers should
    /// prefer this field over `provider_tenant_id`.
    pub provider_tenant_ids: std::vec::Vec<String>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

pub struct Discovery {
    http_client: crate::client::Client,
}

impl Discovery {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/b2b/oauth/discovery/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
