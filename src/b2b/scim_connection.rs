// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::scim::SCIMConnection;
use crate::b2b::scim::SCIMConnectionWithNextToken;
use crate::b2b::scim::SCIMConnectionWithToken;
use crate::b2b::scim::SCIMGroupImplicitRoleAssignments;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    pub organization_id: String,
    pub display_name: std::option::Option<String>,
    pub identity_provider: std::option::Option<CreateRequestIdentityProvider>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnectionWithToken>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    pub organization_id: String,
    pub connection_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub request_id: String,
    pub connection_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    pub organization_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnection>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateCancelRequest {
    pub organization_id: String,
    pub connection_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateCancelResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnection>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateCompleteRequest {
    pub organization_id: String,
    pub connection_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateCompleteResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnection>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateStartRequest {
    pub organization_id: String,
    pub connection_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateStartResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnectionWithNextToken>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    pub organization_id: String,
    pub connection_id: String,
    pub display_name: std::option::Option<String>,
    pub identity_provider: std::option::Option<UpdateRequestIdentityProvider>,
    pub scim_group_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SCIMGroupImplicitRoleAssignments>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<SCIMConnection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CreateRequestIdentityProvider {
    #[serde(rename = "generic")]
    #[default]
    Generic,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UpdateRequestIdentityProvider {
    #[serde(rename = "generic")]
    #[default]
    Generic,
    #[serde(rename = "okta")]
    Okta,
    #[serde(rename = "microsoftentra")]
    Microsoftentra,
    #[serde(rename = "cyberark")]
    Cyberark,
    #[serde(rename = "jumpcloud")]
    Jumpcloud,
    #[serde(rename = "onelogin")]
    Onelogin,
    #[serde(rename = "pingfederate")]
    Pingfederate,
    #[serde(rename = "rippling")]
    Rippling,
}

pub struct Connection {
    http_client: crate::client::Client,
}

impl Connection {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}");
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
        let connection_id = &body.connection_id;
        let path = format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_start(
        &self,
        body: RotateStartRequest,
    ) -> crate::Result<RotateStartResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/start");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_complete(
        &self,
        body: RotateCompleteRequest,
    ) -> crate::Result<RotateCompleteResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/complete");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn rotate_cancel(
        &self,
        body: RotateCancelRequest,
    ) -> crate::Result<RotateCancelResponse> {
        let organization_id = &body.organization_id;
        let connection_id = &body.connection_id;
        let path =
            format!("/v1/b2b/scim/{organization_id}/connection/{connection_id}/rotate/cancel");
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
        let path = format!("/v1/b2b/scim/{organization_id}/connection");
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
        let path = format!("/v1/b2b/scim/{organization_id}/connection");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}