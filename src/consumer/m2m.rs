// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::m2m_clients::Clients;
use serde::{Deserialize, Serialize};

/// M2MClient:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct M2MClient {
    /// client_id: The ID of the client.
    pub client_id: String,
    /// client_name: A human-readable name for the client.
    pub client_name: String,
    /// client_description: A human-readable description for the client.
    pub client_description: String,
    /// status: The status of the client - either `active` or `inactive`.
    pub status: String,
    /// scopes: An array of scopes assigned to the client.
    pub scopes: std::vec::Vec<String>,
    /// client_secret_last_four: The last four characters of the client secret.
    pub client_secret_last_four: String,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data.
    pub trusted_metadata: std::option::Option<String>,
    /// next_client_secret_last_four: The last four characters of the `next_client_secret`. Null if no
    /// `next_client_secret` exists.
    pub next_client_secret_last_four: std::option::Option<String>,
}

/// M2MClientWithClientSecret:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct M2MClientWithClientSecret {
    /// client_id: The ID of the client.
    pub client_id: String,
    /// client_secret: The secret of the client. **Important:** this is the only time you will be able to view
    /// the `client_secret`. Be sure to persist the `client_secret` in a secure location. If the `client_secret`
    /// is lost, you will need to trigger a secret rotation flow to receive another one.
    pub client_secret: String,
    /// client_name: A human-readable name for the client.
    pub client_name: String,
    /// client_description: A human-readable description for the client.
    pub client_description: String,
    /// status: The status of the client - either `active` or `inactive`.
    pub status: String,
    /// scopes: An array of scopes assigned to the client.
    pub scopes: std::vec::Vec<String>,
    /// client_secret_last_four: The last four characters of the client secret.
    pub client_secret_last_four: String,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data.
    pub trusted_metadata: std::option::Option<String>,
    /// next_client_secret_last_four: The last four characters of the `next_client_secret`. Null if no
    /// `next_client_secret` exists.
    pub next_client_secret_last_four: std::option::Option<String>,
}

/// M2MClientWithNextClientSecret:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct M2MClientWithNextClientSecret {
    /// client_id: The ID of the client.
    pub client_id: String,
    /// next_client_secret: The newly created secret that's next in rotation for the client. **Important:** this
    /// is the only time you will be able to view the `next_client_secret`. Be sure to persist the
    /// `next_client_secret` in a secure location. If the `next_client_secret` is lost, you will need to trigger
    /// a secret rotation flow to receive another one.
    pub next_client_secret: String,
    /// client_name: A human-readable name for the client.
    pub client_name: String,
    /// client_description: A human-readable description for the client.
    pub client_description: String,
    /// status: The status of the client - either `active` or `inactive`.
    pub status: String,
    /// scopes: An array of scopes assigned to the client.
    pub scopes: std::vec::Vec<String>,
    /// client_secret_last_four: The last four characters of the client secret.
    pub client_secret_last_four: String,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data.
    pub trusted_metadata: std::option::Option<String>,
    /// next_client_secret_last_four: The last four characters of the `next_client_secret`. Null if no
    /// `next_client_secret` exists.
    pub next_client_secret_last_four: std::option::Option<String>,
}

/// M2MSearchQuery:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct M2MSearchQuery {
    /// operator: The action to perform on the operands. The accepted value are:
    ///
    ///   `AND` – all the operand values provided must match.
    ///   
    ///   `OR` – the operator will return any matches to at least one of the operand values you supply.
    pub operator: M2MSearchQueryOperator,
    /// operands: An array of operand objects that contains all of the filters and values to apply to your
    /// search search query.
    pub operands: std::vec::Vec<String>,
}

/// ResultsMetadata:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultsMetadata {
    /// total: The total number of results returned by your search query.
    pub total: i32,
    /// next_cursor: The `next_cursor` string is returned when your search result contains more than one page of
    /// results. This value is passed into your next search call in the `cursor` field.
    pub next_cursor: std::option::Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum M2MSearchQueryOperator {
    #[serde(rename = "or")]
    OR,
    #[serde(rename = "and")]
    AND,
}

pub struct M2M {
    pub clients: Clients,
}

impl M2M {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            clients: Clients::new(http_client.clone()),
        }
    }
}
