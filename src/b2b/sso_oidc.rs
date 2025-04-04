// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::sso::OIDCConnection;
use serde::{Deserialize, Serialize};

/// CreateConnectionRequest: Request type for `OIDC.create_connection`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConnectionRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value. You may also use
    /// the organization_slug here as a convenience.
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
/// CreateConnectionResponse: Response type for `OIDC.create_connection`.
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
    /// connection: The `OIDC Connection` object affected by this API call. See the
    /// [OIDC Connection Object](https://stytch.com/docs/b2b/api/oidc-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<OIDCConnection>,
}
/// UpdateConnectionRequest: Request type for `OIDC.update_connection`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateConnectionRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value. You may also use
    /// the organization_slug here as a convenience.
    pub organization_id: String,
    /// connection_id: Globally unique UUID that identifies a specific SSO `connection_id` for a Member.
    pub connection_id: String,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
    /// client_id: The OAuth2.0 client ID used to authenticate login attempts. This will be provided by the IdP.
    pub client_id: std::option::Option<String>,
    /// client_secret: The secret belonging to the OAuth2.0 client used to authenticate login attempts. This
    /// will be provided by the IdP.
    pub client_secret: std::option::Option<String>,
    /// issuer: A case-sensitive `https://` URL that uniquely identifies the IdP. This will be provided by the
    /// IdP.
    pub issuer: std::option::Option<String>,
    /// authorization_url: The location of the URL that starts an OAuth login at the IdP. This will be provided
    /// by the IdP.
    pub authorization_url: std::option::Option<String>,
    /// token_url: The location of the URL that issues OAuth2.0 access tokens and OIDC ID tokens. This will be
    /// provided by the IdP.
    pub token_url: std::option::Option<String>,
    /// userinfo_url: The location of the IDP's
    /// [UserInfo Endpoint](https://openid.net/specs/openid-connect-core-1_0.html#UserInfo). This will be
    /// provided by the IdP.
    pub userinfo_url: std::option::Option<String>,
    /// jwks_url: The location of the IdP's JSON Web Key Set, used to verify credentials issued by the IdP. This
    /// will be provided by the IdP.
    pub jwks_url: std::option::Option<String>,
    /// identity_provider: Name of the IdP. Enum with possible values: `classlink`, `cyberark`, `duo`,
    /// `google-workspace`, `jumpcloud`, `keycloak`, `miniorange`, `microsoft-entra`, `okta`, `onelogin`,
    /// `pingfederate`, `rippling`, `salesforce`, `shibboleth`, or `generic`.
    ///
    /// Specifying a known provider allows Stytch to handle any provider-specific logic.
    pub identity_provider: std::option::Option<UpdateConnectionRequestIdentityProvider>,
    /// custom_scopes: Include a space-separated list of custom scopes that you'd like to include. Note that
    /// this list must be URL encoded, e.g. the spaces must be expressed as %20.
    pub custom_scopes: std::option::Option<String>,
    /// attribute_mapping: An object that represents the attributes used to identify a Member. This object will
    /// map the IdP-defined User attributes to Stytch-specific values, which will appear on the member's Trusted
    /// Metadata.
    pub attribute_mapping: std::option::Option<serde_json::Value>,
}
/// UpdateConnectionResponse: Response type for `OIDC.update_connection`.
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
    /// connection: The `OIDC Connection` object affected by this API call. See the
    /// [OIDC Connection Object](https://stytch.com/docs/b2b/api/oidc-connection-object) for complete response
    /// field details.
    pub connection: std::option::Option<OIDCConnection>,
    /// warning: If it is not possible to resolve the well-known metadata document from the OIDC issuer, this
    /// field will explain what went wrong if the request is successful otherwise. In other words, even if the
    /// overall request succeeds, there could be relevant warnings related to the connection update.
    pub warning: std::option::Option<String>,
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

pub struct OIDC {
    http_client: crate::client::Client,
}

impl OIDC {
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
        let path = format!("/v1/b2b/sso/oidc/{organization_id}");
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
        let path = format!("/v1/b2b/sso/oidc/{organization_id}/connections/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
}
