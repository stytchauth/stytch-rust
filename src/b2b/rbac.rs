// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use serde::{Deserialize, Serialize};

/// Policy:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Policy {
    /// roles: An array of [Role objects](https://stytch.com/docs/b2b/api/rbac-role-object).
    pub roles: std::vec::Vec<PolicyRole>,
    /// resources: An array of [Resource objects](https://stytch.com/docs/b2b/api/rbac-resource-object).
    pub resources: std::vec::Vec<PolicyResource>,
}
/// PolicyResource:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyResource {
    /// resource_id: A unique identifier of the RBAC Resource, provided by the developer and intended to be
    /// human-readable.
    ///
    ///   A `resource_id` is not allowed to start with `stytch`, which is a special prefix used for Stytch
    /// default Resources with reserved  `resource_id`s. These include:
    ///
    ///   * `stytch.organization`
    ///   * `stytch.member`
    ///   * `stytch.sso`
    ///   * `stytch.self`
    ///
    ///   Check out the
    /// [guide on Stytch default Resources](https://stytch.com/docs/b2b/guides/rbac/stytch-default) for a more
    /// detailed explanation.
    ///
    ///
    pub resource_id: String,
    /// description: The description of the RBAC Resource.
    pub description: String,
    /// actions: A list of all possible actions for a provided Resource.
    ///
    ///   Reserved `actions` that are predefined by Stytch include:
    ///
    ///   * `*`
    ///   * For the `stytch.organization` Resource:
    /// * `update.info.name`
    /// * `update.info.slug`
    /// * `update.info.untrusted_metadata`
    /// * `update.info.email_jit_provisioning`
    /// * `update.info.logo_url`
    /// * `update.info.email_invites`
    /// * `update.info.allowed_domains`
    /// * `update.info.default_sso_connection`
    /// * `update.info.sso_jit_provisioning`
    /// * `update.info.mfa_policy`
    /// * `update.info.implicit_roles`
    /// * `delete`
    ///   * For the `stytch.member` Resource:
    /// * `create`
    /// * `update.info.name`
    /// * `update.info.untrusted_metadata`
    /// * `update.info.mfa-phone`
    /// * `update.info.delete.mfa-phone`
    /// * `update.settings.is-breakglass`
    /// * `update.settings.mfa_enrolled`
    /// * `update.settings.roles`
    /// * `search`
    /// * `delete`
    ///   * For the `stytch.sso` Resource:
    /// * `create`
    /// * `update`
    /// * `delete`
    ///   * For the `stytch.self` Resource:
    /// * `update.info.name`
    /// * `update.info.untrusted_metadata`
    /// * `update.info.mfa-phone`
    /// * `update.info.delete.mfa-phone`
    /// * `update.info.delete.password`
    /// * `update.settings.mfa_enrolled`
    /// * `delete`
    ///
    pub actions: std::vec::Vec<String>,
}
/// PolicyRole:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyRole {
    /// role_id: The unique identifier of the RBAC Role, provided by the developer and intended to be
    /// human-readable.  
    ///
    ///   Reserved `role_id`s that are predefined by Stytch include:
    ///
    ///   * `stytch_member`
    ///   * `stytch_admin`
    ///
    ///   Check out the [guide on Stytch default Roles](https://stytch.com/docs/b2b/guides/rbac/stytch-default)
    /// for a more detailed explanation.
    ///
    ///
    pub role_id: String,
    /// description: The description of the RBAC Role.
    pub description: String,
    /// permissions: A list of permissions that link a
    /// [Resource](https://stytch.com/docs/b2b/api/rbac-resource-object) to a list of actions.
    pub permissions: std::vec::Vec<PolicyRolePermission>,
}
/// PolicyRolePermission:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyRolePermission {
    /// resource_id: A unique identifier of the RBAC Resource, provided by the developer and intended to be
    /// human-readable.
    ///
    ///   A `resource_id` is not allowed to start with `stytch`, which is a special prefix used for Stytch
    /// default Resources with reserved  `resource_id`s. These include:
    ///
    ///   * `stytch.organization`
    ///   * `stytch.member`
    ///   * `stytch.sso`
    ///   * `stytch.self`
    ///
    ///   Check out the
    /// [guide on Stytch default Resources](https://stytch.com/docs/b2b/guides/rbac/stytch-default) for a more
    /// detailed explanation.
    ///
    ///
    pub resource_id: String,
    /// actions: A list of permitted actions the Role is authorized to take with the provided Resource. You can
    /// use `*` as a wildcard to grant a Role permission to use all possible actions related to the Resource.
    pub actions: std::vec::Vec<String>,
}
/// PolicyRequest: Request type for `RBAC.policy`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PolicyRequest {}
/// PolicyResponse: Response type for `RBAC.policy`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// policy: The RBAC Policy document that contains all defined Roles and Resources – which are managed in
    /// the [Dashboard](/dashboard/rbac). Read more about these entities and how they work in our
    /// [RBAC overview](https://stytch.com/docs/b2b/guides/rbac/overview).
    pub policy: std::option::Option<Policy>,
}

pub struct RBAC {
    http_client: crate::client::Client,
}

impl RBAC {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn policy(&self, body: PolicyRequest) -> crate::Result<PolicyResponse> {
        let path = String::from("/v1/b2b/rbac/policy");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}
