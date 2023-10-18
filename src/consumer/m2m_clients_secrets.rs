// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::m2m::M2MClient;
use crate::consumer::m2m::M2MClientWithNextClientSecret;
use serde::{Deserialize, Serialize};

/// RotateCancelRequest: Request type for `Secrets.rotate_cancel`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateCancelRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}

/// RotateCancelResponse: Response type for `Secrets.rotate_cancel`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateCancelResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client affected by this operation.
    pub m2m_client: M2MClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// RotateRequest: Request type for `Secrets.rotate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}

/// RotateResponse: Response type for `Secrets.rotate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client affected by this operation.
    pub m2m_client: M2MClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// RotateStartRequest: Request type for `Secrets.rotate_start`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RotateStartRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}

/// RotateStartResponse: Response type for `Secrets.rotate_start`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RotateStartResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client affected by this operation.
    pub m2m_client: M2MClientWithNextClientSecret,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

pub struct Secrets {
    http_client: crate::client::Client,
}

impl Secrets {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn rotate_start(
        &self,
        body: RotateStartRequest,
    ) -> crate::Result<RotateStartResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}/secrets/rotate/start");
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
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}/secrets/rotate/cancel");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn rotate(&self, body: RotateRequest) -> crate::Result<RotateResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}/secrets/rotate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
