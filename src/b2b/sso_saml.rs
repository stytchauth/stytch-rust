// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::sso::SAMLConnection;
use serde::{Deserialize, Serialize};

/// CreateConnectionRequest: Request type for `SAML.create_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateConnectionRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// display_name: A human-readable display name for the connection.
    pub display_name: std::option::Option<String>,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// UpdateConnectionRequest: Request type for `SAML.update_connection`.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub attribute_mapping: std::option::Option<String>,
    /// x509_certificate: A certificate that Stytch will use to verify the sign-in assertion sent by the IdP, in
    /// [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. See our
    /// [X509 guide](https://stytch.com/docs/b2b/api/saml-certificates) for more info.
    pub x509_certificate: std::option::Option<String>,
    /// idp_sso_url: The URL for which assertions for login requests will be sent. This will be provided by the
    /// IdP.
    pub idp_sso_url: std::option::Option<String>,
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

pub struct SAML {
    http_client: crate::reqwest::Client,
}

impl SAML {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
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
