// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::fraud::VerdictReasonAction;
use serde::{Deserialize, Serialize};

/// ListRequest: Request type for `VerdictReasons.list`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ListRequest {
    /// overrides_only: Whether to return only verdict reasons that have overrides set. Defaults to false.
    pub overrides_only: std::option::Option<bool>,
}
/// ListResponse: Response type for `VerdictReasons.list`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// verdict_reason_actions: Information about verdict reasons and any overrides that were set on them.
    pub verdict_reason_actions: std::vec::Vec<VerdictReasonAction>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// OverrideRequest: Request type for `VerdictReasons.override`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OverrideRequest {
    /// verdict_reason: The verdict reason that you wish to override. For a list of possible reasons to
    /// override, see
    /// [Warning Flags (Verdict Reasons)](https://stytch.com/docs/docs/fraud/guides/device-fingerprinting/reference/warning-flags-verdict-reasons). You may not override the `RULE_MATCH` reason.
    pub verdict_reason: String,
    /// override_action: The action that you want to be returned for the specified verdict reason. The override
    /// action must be one of `ALLOW`, `BLOCK`, or `CHALLENGE`.
    pub override_action: OverrideRequestAction,
    /// override_description: An optional description for the verdict reason override.
    pub override_description: std::option::Option<String>,
}
/// OverrideResponse: Response type for `VerdictReasons.override`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverrideResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// verdict_reason_action: Information about the verdict reason override that was just set.
    pub verdict_reason_action: VerdictReasonAction,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum OverrideRequestAction {
    #[serde(rename = "ALLOW")]
    #[default]
    ALLOW,
    #[serde(rename = "CHALLENGE")]
    CHALLENGE,
    #[serde(rename = "BLOCK")]
    BLOCK,
    #[serde(rename = "NONE")]
    NONE,
}

pub struct VerdictReasons {
    http_client: crate::client::Client,
}

impl VerdictReasons {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn r#override(&self, body: OverrideRequest) -> crate::Result<OverrideResponse> {
        let path = String::from("/v1/verdict_reasons/override");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn list(&self, body: ListRequest) -> crate::Result<ListResponse> {
        let path = String::from("/v1/verdict_reasons/list");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
