// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::sso::SAMLConnection;
use crate::b2b::sso::SAMLConnectionImplicitRoleAssignment;
use crate::b2b::sso::SAMLGroupImplicitRoleAssignment;
use serde::{Deserialize, Serialize};

/// CreateConnectionRequest: Request type for `SAML.create_connection`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConnectionRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
    /// identity_provider: Name of the IdP. Enum with possible values: `classlink`, `cyberark`, `duo`,
    /// `google-workspace`, `jumpcloud`, `keycloak`, `miniorange`, `microsoft-entra`, `okta`, `onelogin`,
    /// `pingfederate`, `rippling`, `salesforce`, `shibboleth`, or `generic`.
    ///
    /// Specifying a known provider allows Stytch to handle any provider-specific logic.
    pub identity_provider: std::option::Option<CreateConnectionRequestIdentityProvider>,
}
/// CreateConnectionResponse: Response type for `SAML.create_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateConnectionResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SAML Connection` object affected by this API call. See the
    /// [SAML Connection Object](https://stytch.com/docs/b2b/api/saml-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SAMLConnection>,
}
/// DeleteVerificationCertificateRequest: Request type for `SAML.delete_verification_certificate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteVerificationCertificateRequest {
    /// organization_id: The organization ID that the SAML connection belongs to.
    pub organization_id: String,
    /// connection_id: The ID of the SAML connection.
    pub connection_id: String,
    /// certificate_id: The ID of the certificate to be deleted.
    pub certificate_id: String,
}
/// DeleteVerificationCertificateResponse: Response type for `SAML.delete_verification_certificate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteVerificationCertificateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// certificate_id: The ID of the certificate that was deleted.
    pub certificate_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// UpdateByURLRequest: Request type for `SAML.update_by_url`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateByURLRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: Globally unique UUID that identifies a specific SSO `connection_id` for a Member.
    pub connection_id: String,
    /// metadata_url: A URL that points to the IdP metadata. This will be provided by the IdP.
    pub metadata_url: String,
}
/// UpdateByURLResponse: Response type for `SAML.update_by_url`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateByURLResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SAML Connection` object affected by this API call. See the
    /// [SAML Connection Object](https://stytch.com/docs/b2b/api/saml-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SAMLConnection>,
}
/// UpdateConnectionRequest: Request type for `SAML.update_connection`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateConnectionRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: Globally unique UUID that identifies a specific SSO `connection_id` for a Member.
    pub connection_id: String,
    /// idp_entity_id: A globally unique name for the IdP. This will be provided by the IdP.
    pub idp_entity_id: std::option::Option<String>,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
    /// attribute_mapping: An object that represents the attributes used to identify a Member. This object will
    /// map the IdP-defined User attributes to Stytch-specific values. Required attributes: `email` and one of
    /// `full_name` or `first_name` and `last_name`.
    pub attribute_mapping: std::option::Option<serde_json::Value>,
    /// x509_certificate: A certificate that Stytch will use to verify the sign-in assertion sent by the IdP, in
    /// [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. See our
    /// [X509 guide](https://stytch.com/docs/b2b/api/saml-certificates) for more info.
    pub x509_certificate: std::option::Option<String>,
    /// idp_sso_url: The URL for which assertions for login requests will be sent. This will be provided by the
    /// IdP.
    pub idp_sso_url: std::option::Option<String>,
    /// saml_connection_implicit_role_assignments: All Members who log in with this SAML connection will
    /// implicitly receive the specified Roles. See the
    /// [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment) for more information about role
    /// assignment.
    pub saml_connection_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SAMLConnectionImplicitRoleAssignment>>,
    /// saml_group_implicit_role_assignments: Defines the names of the SAML groups
    ///  that grant specific role assignments. For each group-Role pair, if a Member logs in with this SAML
    /// connection and
    ///  belongs to the specified SAML group, they will be granted the associated Role. See the
    ///  [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment) for more information about role
    /// assignment. Before adding any group implicit role assignments, you must add a "groups" key to your SAML
    /// connection's
    ///  `attribute_mapping`. Make sure that your IdP is configured to correctly send the group information.
    pub saml_group_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SAMLGroupImplicitRoleAssignment>>,
    /// alternative_audience_uri: An alternative URL to use for the Audience Restriction. This value can be used
    /// when you wish to migrate an existing SAML integration to Stytch with zero downtime. Read our
    /// [SSO migration guide](https://stytch.com/docs/b2b/guides/migrations/additional-migration-considerations)
    /// for more info.
    pub alternative_audience_uri: std::option::Option<String>,
    /// identity_provider: Name of the IdP. Enum with possible values: `classlink`, `cyberark`, `duo`,
    /// `google-workspace`, `jumpcloud`, `keycloak`, `miniorange`, `microsoft-entra`, `okta`, `onelogin`,
    /// `pingfederate`, `rippling`, `salesforce`, `shibboleth`, or `generic`.
    ///
    /// Specifying a known provider allows Stytch to handle any provider-specific logic.
    pub identity_provider: std::option::Option<UpdateConnectionRequestIdentityProvider>,
    /// signing_private_key: A PKCS1 format RSA private key used for signing SAML requests. Only PKCS1 format
    /// (starting with "-----BEGIN RSA PRIVATE KEY-----") is supported. When provided, Stytch will generate a
    /// new x509 certificate from this key and return it in the signing_certificates array.
    pub signing_private_key: std::option::Option<String>,
}
/// UpdateConnectionResponse: Response type for `SAML.update_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateConnectionResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// connection: The `SAML Connection` object affected by this API call. See the
    /// [SAML Connection Object](https://stytch.com/docs/b2b/api/saml-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<SAMLConnection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CreateConnectionRequestIdentityProvider {
    #[serde(rename = "classlink")]
    #[default]
    Classlink,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "duo")]
    Duo,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "googleworkspace")]
    Googleworkspace,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "keycloak")]
    Keycloak,
    #[serde(rename = "miniorange")]
    Miniorange,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
    #[serde(rename = "salesforce")]
    Salesforce,
    #[serde(rename = "shibboleth")]
    Shibboleth,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UpdateConnectionRequestIdentityProvider {
    #[serde(rename = "classlink")]
    #[default]
    Classlink,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "duo")]
    Duo,
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "googleworkspace")]
    Googleworkspace,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "keycloak")]
    Keycloak,
    #[serde(rename = "miniorange")]
    Miniorange,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
    #[serde(rename = "salesforce")]
    Salesforce,
    #[serde(rename = "shibboleth")]
    Shibboleth,
}

pub struct SAML {
    http_client: crate::client::Client,
}

impl SAML {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn create_connection(
        &self,
        body: CreateConnectionRequest,
    ) -> crate::Result<CreateConnectionResponse> {
        let organization_id = &body.organization_id;
        let path = format!("/v1/b2b/sso/saml/{organization_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn update_connection(
        &self,
        body: UpdateConnectionRequest,
    ) -> crate::Result<UpdateConnectionResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/sso/saml/{organization_id}/connections/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn update_by_url(
        &self,
        body: UpdateByURLRequest,
    ) -> crate::Result<UpdateByURLResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/sso/saml/{organization_id}/connections/{connection_id}/url");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete_verification_certificate(
        &self,
        body: DeleteVerificationCertificateRequest,
    ) -> crate::Result<DeleteVerificationCertificateResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let certificate_id = &body.certificate_id;
        let path = format!("/v1/b2b/sso/saml/{organization_id}/connections/{connection_id}/verification_certificates/{certificate_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
}
