// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::scim_connection::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub formatted: String,
    pub street_address: String,
    pub locality: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub primary: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub primary: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnterpriseExtension {
    pub employee_number: String,
    pub cost_center: String,
    pub division: String,
    pub department: String,
    pub organization: String,
    pub manager: std::option::Option<Manager>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub value: String,
    pub display: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manager {
    pub value: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub display_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    pub formatted: String,
    pub family_name: String,
    pub given_name: String,
    pub middle_name: String,
    pub honorific_prefix: String,
    pub honorific_suffix: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneNumber {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub primary: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMAttributes {
    pub user_name: String,
    pub id: String,
    pub external_id: String,
    pub active: bool,
    pub groups: std::vec::Vec<Group>,
    pub display_name: String,
    pub nick_name: String,
    pub profile_url: String,
    pub user_type: String,
    pub title: String,
    pub preferred_language: String,
    pub locale: String,
    pub timezone: String,
    pub emails: std::vec::Vec<Email>,
    pub phone_numbers: std::vec::Vec<PhoneNumber>,
    pub addresses: std::vec::Vec<Address>,
    pub name: std::option::Option<Name>,
    pub enterprise_extension: std::option::Option<EnterpriseExtension>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMConnection {
    pub organization_id: String,
    pub connection_id: String,
    pub status: String,
    pub display_name: String,
    pub identity_provider: String,
    pub base_url: String,
    pub bearer_token_last_four: String,
    pub scim_group_implicit_role_assignments: std::vec::Vec<SCIMGroupImplicitRoleAssignments>,
    pub next_bearer_token_last_four: String,
    pub bearer_token_expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    pub next_bearer_token_expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMConnectionWithNextToken {
    pub organization_id: String,
    pub connection_id: String,
    pub status: String,
    pub display_name: String,
    pub base_url: String,
    pub identity_provider: String,
    pub bearer_token_last_four: String,
    pub next_bearer_token: String,
    pub scim_group_implicit_role_assignments: std::vec::Vec<SCIMGroupImplicitRoleAssignments>,
    pub bearer_token_expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    pub next_bearer_token_expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMConnectionWithToken {
    pub organization_id: String,
    pub connection_id: String,
    pub status: String,
    pub display_name: String,
    pub identity_provider: String,
    pub base_url: String,
    pub bearer_token: String,
    pub scim_group_implicit_role_assignments: std::vec::Vec<SCIMGroupImplicitRoleAssignments>,
    pub bearer_token_expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
}
/// SCIMGroup:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMGroup {
    /// group_id: Globally unique UUID that identifies a specific SCIM Group.
    pub group_id: String,
    /// group_name: The name of the SCIM group.
    pub group_name: String,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The organization_id is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// connection_id: The ID of the SCIM connection.
    pub connection_id: String,
}
/// SCIMGroupImplicitRoleAssignments:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SCIMGroupImplicitRoleAssignments {
    /// role_id: The ID of the role.
    pub role_id: String,
    pub group_id: String,
    pub group_name: String,
}

pub struct SCIM {
    pub connection: Connection,
}

impl SCIM {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            connection: Connection::new(http_client.clone()),
        }
    }
}
