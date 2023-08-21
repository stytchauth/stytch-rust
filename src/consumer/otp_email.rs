// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use serde::{Deserialize, Serialize};

/// LoginOrCreateRequest: Request type for `Email.login_or_create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginOrCreateRequest {
    /// email: The email address of the user to send the one-time passcode to. You may use sandbox@stytch.com to
    /// test this endpoint, see [Testing](https://stytch.com/docs/home#resources_testing) for more detail.
    pub email: String,
    /// expiration_minutes: Set the expiration for the one-time passcode, in minutes. The minimum expiration is
    /// 1 minute and the maximum is 10 minutes. The default expiration is 2 minutes.
    pub expiration_minutes: std::option::Option<i32>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// create_user_as_pending: Flag for whether or not to save a user as pending vs active in Stytch. Defaults
    /// to false.
    ///         If true, users will be saved with status pending in Stytch's backend until authenticated.
    ///         If false, users will be created as active. An example usage of
    ///         a true flag would be to require users to verify their phone by entering the OTP code before
    /// creating
    ///         an account for them.
    pub create_user_as_pending: std::option::Option<bool>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<LoginOrCreateRequestLocale>,
    /// login_template_id: Use a custom template for login emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Login.
    pub login_template_id: std::option::Option<String>,
    /// signup_template_id: Use a custom template for sign-up emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Sign-up.
    pub signup_template_id: std::option::Option<String>,
}

/// LoginOrCreateResponse: Response type for `Email.login_or_create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginOrCreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// email_id: The unique ID of a specific email address.
    pub email_id: String,
    /// user_created: In `login_or_create` endpoints, this field indicates whether or not a User was just
    /// created.
    pub user_created: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// SendRequest: Request type for `Email.send`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendRequest {
    /// email: The email address of the user to send the one-time passcode to. You may use sandbox@stytch.com to
    /// test this endpoint, see [Testing](https://stytch.com/docs/home#resources_testing) for more detail.
    pub email: String,
    /// expiration_minutes: Set the expiration for the one-time passcode, in minutes. The minimum expiration is
    /// 1 minute and the maximum is 10 minutes. The default expiration is 2 minutes.
    pub expiration_minutes: std::option::Option<i32>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<SendRequestLocale>,
    /// user_id: The unique ID of a specific User.
    pub user_id: std::option::Option<String>,
    /// session_token: The `session_token` associated with a User's existing Session.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The `session_jwt` associated with a User's existing Session.
    pub session_jwt: std::option::Option<String>,
    /// login_template_id: Use a custom template for login emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Login.
    pub login_template_id: std::option::Option<String>,
    /// signup_template_id: Use a custom template for sign-up emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Sign-up.
    pub signup_template_id: std::option::Option<String>,
}

/// SendResponse: Response type for `Email.send`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// email_id: The unique ID of a specific email address.
    pub email_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginOrCreateRequestLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SendRequestLocale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Email {
    http_client: crate::reqwest::Client,
}

impl Email {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn send(&self, body: SendRequest) -> crate::Result<SendResponse> {
        let path = format!("/v1/otps/email/send");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn login_or_create(
        &self,
        body: LoginOrCreateRequest,
    ) -> crate::Result<LoginOrCreateResponse> {
        let path = format!("/v1/otps/email/login_or_create");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
