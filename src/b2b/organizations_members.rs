// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::organizations::ResultsMetadata;
use crate::b2b::organizations::SearchQuery;
use crate::b2b::organizations_members_oauth_providers::OAuthProviders;
use serde::{Deserialize, Serialize};

/// CreateRequest: Request type for `Members.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// email_address: The email address of the Member.
    pub email_address: String,
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
    /// create_member_as_pending: Flag for whether or not to save a Member as `pending` or `active` in Stytch.
    /// It defaults to false. If true, new Members will be created with status `pending` in Stytch's backend.
    /// Their status will remain `pending` and they will continue to receive signup email templates for every
    /// Email Magic Link until that Member authenticates and becomes `active`. If false, new Members will be
    /// created with status `active`.
    pub create_member_as_pending: std::option::Option<bool>,
    /// is_breakglass: Identifies the Member as a break glass user - someone who has permissions to authenticate
    /// into an Organization by bypassing the Organization's settings. A break glass account is typically used
    /// for emergency purposes to gain access outside of normal authentication procedures. Refer to the
    /// [Organization object](organization-object) and its `auth_methods` and `allowed_auth_methods` fields for
    /// more details.
    pub is_breakglass: std::option::Option<bool>,
    /// mfa_phone_number: The Member's phone number. A Member may only have one phone number.
    pub mfa_phone_number: std::option::Option<String>,
    /// mfa_enrolled: Sets whether the Member is enrolled in MFA. If true, the Member must complete an MFA step
    /// whenever they wish to log in to their Organization. If false, the Member only needs to complete an MFA
    /// step if the Organization's MFA policy is set to `REQUIRED_FOR_ALL`.
    pub mfa_enrolled: std::option::Option<bool>,
    /// roles: Roles to explicitly assign to this Member. See the
    /// [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment)
    ///    for more information about role assignment.
    pub roles: std::option::Option<std::vec::Vec<String>>,
}
/// CreateResponse: Response type for `Members.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
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
/// DangerouslyGetRequest: Request type for `Members.dangerously_get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DangerouslyGetRequest {
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}
/// DeleteMFAPhoneNumberRequest: Request type for `Members.delete_mfa_phone_number`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteMFAPhoneNumberRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}
/// DeleteMFAPhoneNumberResponse: Response type for `Members.delete_mfa_phone_number`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMFAPhoneNumberResponse {
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
/// DeletePasswordRequest: Request type for `Members.delete_password`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeletePasswordRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_password_id: Globally unique UUID that identifies a Member's password.
    pub member_password_id: String,
}
/// DeletePasswordResponse: Response type for `Members.delete_password`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePasswordResponse {
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
/// DeleteRequest: Request type for `Members.delete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}
/// DeleteResponse: Response type for `Members.delete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// DeleteTOTPRequest: Request type for `Members.delete_totp`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteTOTPRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}
/// DeleteTOTPResponse: Response type for `Members.delete_totp`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteTOTPResponse {
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
/// GetRequest: Request type for `Members.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: std::option::Option<String>,
    /// email_address: The email address of the Member.
    pub email_address: std::option::Option<String>,
}
/// GetResponse: Response type for `Members.dangerously_get`, `Members.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
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
/// ReactivateRequest: Request type for `Members.reactivate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReactivateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
}
/// ReactivateResponse: Response type for `Members.reactivate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactivateResponse {
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
/// SearchRequest: Request type for `Members.search`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchRequest {
    /// organization_ids: An array of organization_ids. At least one value is required.
    pub organization_ids: std::vec::Vec<String>,
    /// cursor: The `cursor` field allows you to paginate through your results. Each result array is limited to
    /// 1000 results. If your query returns more than 1000 results, you will need to paginate the responses
    /// using the `cursor`. If you receive a response that includes a non-null `next_cursor` in the
    /// `results_metadata` object, repeat the search call with the `next_cursor` value set to the `cursor` field
    /// to retrieve the next page of results. Continue to make search calls until the `next_cursor` in the
    /// response is null.
    pub cursor: std::option::Option<String>,
    /// limit: The number of search results to return per page. The default limit is 100. A maximum of 1000
    /// results can be returned by a single search request. If the total size of your result set is greater than
    /// one page size, you must paginate the response. See the `cursor` field.
    pub limit: std::option::Option<u32>,
    /// query: The optional query object contains the operator, i.e. `AND` or `OR`, and the operands that will
    /// filter your results. Only an operator is required. If you include no operands, no filtering will be
    /// applied. If you include no query object, it will return all Members with no filtering applied.
    pub query: std::option::Option<SearchQuery>,
}
/// SearchResponse: Response type for `Members.search`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// members: An array of [Member objects](member-object).
    pub members: std::vec::Vec<Member>,
    /// results_metadata: The search `results_metadata` object contains metadata relevant to your specific query
    /// like `total` and `next_cursor`.
    pub results_metadata: ResultsMetadata,
    /// organizations: A map from `organization_id` to
    /// [Organization object](https://stytch.com/docs/b2b/api/organization-object). The map only contains the
    /// Organizations that the Members belongs to.
    pub organizations: std::collections::HashMap<String, Organization>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// UnlinkRetiredEmailRequest: Request type for `Members.unlink_retired_email`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UnlinkRetiredEmailRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
    /// email_id: The globally unique UUID of a Member's email.
    pub email_id: std::option::Option<String>,
    /// email_address: The email address of the Member.
    pub email_address: std::option::Option<String>,
}
/// UnlinkRetiredEmailResponse: Response type for `Members.unlink_retired_email`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnlinkRetiredEmailResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
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
/// UpdateRequest: Request type for `Members.update`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
    /// name: The name of the Member.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.info.name` action on the `stytch.member` Resource. Alternatively, if
    /// the Member Session matches the Member associated with the `member_id` passed in the request, the
    /// authorization check will also allow a Member Session that has permission to perform the
    /// `update.info.name` action on the `stytch.self` Resource.
    pub name: std::option::Option<String>,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data or
    /// identity-provider-specific data.
    ///   If a session header is passed into the request, this field may **not** be passed into the request. You
    /// cannot
    ///   update trusted metadata when acting as a Member.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: An arbitrary JSON object of application-specific data. These fields can be edited
    /// directly by the
    ///   frontend SDK, and should not be used to store critical information. See the
    /// [Metadata resource](https://stytch.com/docs/b2b/api/metadata)
    ///   for complete field behavior details.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.info.untrusted-metadata` action on the `stytch.member` Resource.
    /// Alternatively, if the Member Session matches the Member associated with the `member_id` passed in the
    /// request, the authorization check will also allow a Member Session that has permission to perform the
    /// `update.info.untrusted-metadata` action on the `stytch.self` Resource.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
    /// is_breakglass: Identifies the Member as a break glass user - someone who has permissions to authenticate
    /// into an Organization by bypassing the Organization's settings. A break glass account is typically used
    /// for emergency purposes to gain access outside of normal authentication procedures. Refer to the
    /// [Organization object](organization-object) and its `auth_methods` and `allowed_auth_methods` fields for
    /// more details.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.settings.is-breakglass` action on the `stytch.member` Resource.
    pub is_breakglass: std::option::Option<bool>,
    /// mfa_phone_number: Sets the Member's phone number. Throws an error if the Member already has a phone
    /// number. To change the Member's phone number, use the
    /// [Delete member phone number endpoint](https://stytch.com/docs/b2b/api/delete-member-mfa-phone-number) to
    /// delete the Member's existing phone number first.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.info.mfa-phone` action on the `stytch.member` Resource. Alternatively,
    /// if the Member Session matches the Member associated with the `member_id` passed in the request, the
    /// authorization check will also allow a Member Session that has permission to perform the
    /// `update.info.mfa-phone` action on the `stytch.self` Resource.
    pub mfa_phone_number: std::option::Option<String>,
    /// mfa_enrolled: Sets whether the Member is enrolled in MFA. If true, the Member must complete an MFA step
    /// whenever they wish to log in to their Organization. If false, the Member only needs to complete an MFA
    /// step if the Organization's MFA policy is set to `REQUIRED_FOR_ALL`.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.settings.mfa-enrolled` action on the `stytch.member` Resource.
    /// Alternatively, if the Member Session matches the Member associated with the `member_id` passed in the
    /// request, the authorization check will also allow a Member Session that has permission to perform the
    /// `update.settings.mfa-enrolled` action on the `stytch.self` Resource.
    pub mfa_enrolled: std::option::Option<bool>,
    /// roles: Roles to explicitly assign to this Member.
    ///  Will completely replace any existing explicitly assigned roles. See the
    ///  [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment) for more information about role
    /// assignment.
    ///
    ///    If a Role is removed from a Member, and the Member is also implicitly assigned this Role from an SSO
    /// connection
    ///    or an SSO group, we will by default revoke any existing sessions for the Member that contain any SSO
    ///    authentication factors with the affected connection ID. You can preserve these sessions by passing in
    /// the
    ///    `preserve_existing_sessions` parameter with a value of `true`.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.settings.roles` action on the `stytch.member` Resource.
    pub roles: std::option::Option<std::vec::Vec<String>>,
    /// preserve_existing_sessions: Whether to preserve existing sessions when explicit Roles that are revoked
    /// are also implicitly assigned
    ///   by SSO connection or SSO group. Defaults to `false` - that is, existing Member Sessions that contain
    /// SSO
    ///   authentication factors with the affected SSO connection IDs will be revoked.
    pub preserve_existing_sessions: std::option::Option<bool>,
    /// default_mfa_method: Sets whether the Member is enrolled in MFA. If true, the Member must complete an MFA
    /// step whenever they wish to log in to their Organization. If false, the Member only needs to complete an
    /// MFA step if the Organization's MFA policy is set to `REQUIRED_FOR_ALL`.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.settings.default-mfa-method` action on the `stytch.member` Resource.
    /// Alternatively, if the Member Session matches the Member associated with the `member_id` passed in the
    /// request, the authorization check will also allow a Member Session that has permission to perform the
    /// `update.settings.default-mfa-method` action on the `stytch.self` Resource.
    pub default_mfa_method: std::option::Option<String>,
    /// email_address: Updates the Member's `email_address`, if provided.
    /// If a Member's email address is changed, other Members in the same Organization cannot use the old email
    /// address, although the Member may update back to their old email address.
    /// A Member's email address can only be useable again by other Members if the Member is deleted.
    ///
    /// If this field is provided and a session header is passed into the request, the Member Session must have
    /// permission to perform the `update.info.email` action on the `stytch.member` Resource. Members cannot
    /// update their own email address.
    pub email_address: std::option::Option<String>,
}
/// UpdateResponse: Response type for `Members.update`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
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

pub struct Members {
    http_client: crate::client::Client,
    pub oauth_providers: OAuthProviders,
}

impl Members {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            oauth_providers: OAuthProviders::new(http_client.clone()),
        }
    }

    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members/{member_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete(&self, body: DeleteRequest) -> crate::Result<DeleteResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members/{member_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn reactivate(&self, body: ReactivateRequest) -> crate::Result<ReactivateResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path =
            format!("/v1/b2b/organizations/{organization_id}/members/{member_id}/reactivate");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete_mfa_phone_number(
        &self,
        body: DeleteMFAPhoneNumberRequest,
    ) -> crate::Result<DeleteMFAPhoneNumberResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!(
            "/v1/b2b/organizations/{organization_id}/members/mfa_phone_numbers/{member_id}"
        );
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_totp(&self, body: DeleteTOTPRequest) -> crate::Result<DeleteTOTPResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members/{member_id}/totp");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn search(&self, body: SearchRequest) -> crate::Result<SearchResponse> {
        let path = String::from("/v1/b2b/organizations/members/search");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn delete_password(
        &self,
        body: DeletePasswordRequest,
    ) -> crate::Result<DeletePasswordResponse> {
        let organization_id = &body.organization_id;
        let member_password_id = &body.member_password_id;
        let path = format!(
            "/v1/b2b/organizations/{organization_id}/members/passwords/{member_password_id}"
        );
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn dangerously_get(&self, body: DangerouslyGetRequest) -> crate::Result<GetResponse> {
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/members/dangerously_get/{member_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn unlink_retired_email(
        &self,
        body: UnlinkRetiredEmailRequest,
    ) -> crate::Result<UnlinkRetiredEmailResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!(
            "/v1/b2b/organizations/{organization_id}/members/{member_id}/unlink_retired_email"
        );
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/member");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}
