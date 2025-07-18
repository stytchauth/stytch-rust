// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::connected_apps::ConnectedAppClient;
use crate::consumer::connected_apps::ConnectedAppWithClientSecret;
use crate::consumer::connected_apps::ResultsMetadata;
use crate::consumer::connected_apps_clients_secrets::Secrets;
use serde::{Deserialize, Serialize};

/// CreateRequest: Request type for `Clients.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// client_type: The type of Connected App. Supported values are `first_party`, `first_party_public`,
    /// `third_party`, and `third_party_public`.
    pub client_type: CreateRequestClientType,
    /// redirect_urls: Array of redirect URI values for use in OAuth Authorization flows.
    pub redirect_urls: std::vec::Vec<String>,
    /// full_access_allowed: Valid for first party clients only. If `true`, an authorization token granted to
    /// this Client can be exchanged for a full Stytch session.
    pub full_access_allowed: bool,
    /// post_logout_redirect_urls: Array of redirect URI values for use in OIDC Logout flows.
    pub post_logout_redirect_urls: std::vec::Vec<String>,
    /// client_name: A human-readable name for the client.
    pub client_name: std::option::Option<String>,
    /// client_description: A human-readable description for the client.
    pub client_description: std::option::Option<String>,
    /// access_token_expiry_minutes: The number of minutes before the access token expires. The default is 60
    /// minutes.
    pub access_token_expiry_minutes: std::option::Option<i32>,
    /// access_token_custom_audience: The custom audience for the access token.
    pub access_token_custom_audience: std::option::Option<String>,
    /// access_token_template_content: The content of the access token custom claims template. The template must
    /// be a valid JSON object.
    pub access_token_template_content: std::option::Option<String>,
    /// logo_url: The logo URL of the Connected App, if any.
    pub logo_url: std::option::Option<String>,
    /// bypass_consent_for_offline_access: Valid for first party clients only. If true, the client does not need
    /// to request explicit user consent for the `offline_access` scope.
    pub bypass_consent_for_offline_access: std::option::Option<bool>,
}
/// CreateResponse: Response type for `Clients.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// connected_app: The Connected App created by this API call.
    pub connected_app: ConnectedAppWithClientSecret,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// DeleteRequest: Request type for `Clients.delete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}
/// DeleteResponse: Response type for `Clients.delete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub request_id: String,
    /// client_id: The ID of the client.
    pub client_id: String,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// GetRequest: Request type for `Clients.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// client_id: The ID of the Connected App client.
    pub client_id: String,
}
/// GetResponse: Response type for `Clients.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// connected_app: The Connected App affected by this operation.
    pub connected_app: ConnectedAppClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// SearchRequest: Request type for `Clients.search`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchRequest {
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
}
/// SearchResponse: Response type for `Clients.search`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    pub connected_apps: std::vec::Vec<ConnectedAppClient>,
    /// results_metadata: The search `results_metadata` object contains metadata relevant to your specific query
    /// like total and `next_cursor`.
    pub results_metadata: ResultsMetadata,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// UpdateRequest: Request type for `Clients.update`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
    /// client_name: A human-readable name for the client.
    pub client_name: std::option::Option<String>,
    /// client_description: A human-readable description for the client.
    pub client_description: std::option::Option<String>,
    /// redirect_urls: Array of redirect URI values for use in OAuth Authorization flows.
    pub redirect_urls: std::option::Option<std::vec::Vec<String>>,
    /// full_access_allowed: Valid for first party clients only. If `true`, an authorization token granted to
    /// this Client can be exchanged for a full Stytch session.
    pub full_access_allowed: std::option::Option<bool>,
    /// access_token_expiry_minutes: The number of minutes before the access token expires. The default is 60
    /// minutes.
    pub access_token_expiry_minutes: std::option::Option<i32>,
    /// access_token_custom_audience: The custom audience for the access token.
    pub access_token_custom_audience: std::option::Option<String>,
    /// access_token_template_content: The content of the access token custom claims template. The template must
    /// be a valid JSON object.
    pub access_token_template_content: std::option::Option<String>,
    /// post_logout_redirect_urls: Array of redirect URI values for use in OIDC Logout flows.
    pub post_logout_redirect_urls: std::option::Option<std::vec::Vec<String>>,
    /// logo_url: The logo URL of the Connected App, if any.
    pub logo_url: std::option::Option<String>,
    /// bypass_consent_for_offline_access: Valid for first party clients only. If true, the client does not need
    /// to request explicit user consent for the `offline_access` scope.
    pub bypass_consent_for_offline_access: std::option::Option<bool>,
}
/// UpdateResponse: Response type for `Clients.update`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// connected_app: The Connected App affected by this operation.
    pub connected_app: ConnectedAppClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum CreateRequestClientType {
    #[serde(rename = "first_party")]
    #[default]
    FirstParty,
    #[serde(rename = "first_party_public")]
    FirstPartyPublic,
    #[serde(rename = "third_party")]
    ThirdParty,
    #[serde(rename = "third_party_public")]
    ThirdPartyPublic,
}

pub struct Clients {
    http_client: crate::client::Client,
    pub secrets: Secrets,
}

impl Clients {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            secrets: Secrets::new(http_client.clone()),
        }
    }

    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/connected_apps/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/connected_apps/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete(&self, body: DeleteRequest) -> crate::Result<DeleteResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/connected_apps/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn search(&self, body: SearchRequest) -> crate::Result<SearchResponse> {
        let path = String::from("/v1/connected_apps/clients/search");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let path = String::from("/v1/connected_apps/clients");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
