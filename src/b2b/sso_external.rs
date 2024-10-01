// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::sso::Connection;
use crate::b2b::sso::ConnectionImplicitRoleAssignment;
use crate::b2b::sso::GroupImplicitRoleAssignment;
use crate::b2b::sso::SAMLConnectionImplicitRoleAssignment;
use crate::b2b::sso::SAMLGroupImplicitRoleAssignment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateConnectionRequest {
    pub organization_id: String,
    pub external_organization_id: String,
    pub external_connection_id: String,
    pub display_name: std::option::Option<String>,
    pub connection_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SAMLConnectionImplicitRoleAssignment>>,
    pub group_implicit_role_assignments:
        std::option::Option<std::vec::Vec<SAMLGroupImplicitRoleAssignment>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateConnectionResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<Connection>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateConnectionRequest {
    pub organization_id: String,
    pub connection_id: String,
    pub display_name: std::option::Option<String>,
    pub external_connection_implicit_role_assignments:
        std::option::Option<std::vec::Vec<ConnectionImplicitRoleAssignment>>,
    pub external_group_implicit_role_assignments:
        std::option::Option<std::vec::Vec<GroupImplicitRoleAssignment>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateConnectionResponse {
    pub request_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub connection: std::option::Option<Connection>,
}

pub struct External {
    http_client: crate::client::Client,
}

impl External {
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
        let path = format!("/v1/b2b/sso/external/{organization_id}");
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
        let path = format!("/v1/b2b/sso/external/{organization_id}/connections/{connection_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
}
