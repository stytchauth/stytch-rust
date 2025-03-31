// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use crate::consumer::users::Name;
use serde::{Deserialize, Serialize};

/// InviteRequest: Request type for `Email.invite`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InviteRequest {
    /// email: The email address of the User to send the invite Magic Link to.
    pub email: String,
    /// invite_template_id: Use a custom template for invite emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Invite.
    pub invite_template_id: std::option::Option<String>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// name: The name of the user. Each field in the name object is optional.
    pub name: std::option::Option<Name>,
    /// invite_magic_link_url: The URL the end user clicks from the Email Magic Link. This should be a URL that
    /// your app receives and parses and subsequently sends an API request to authenticate the Magic Link and
    /// log in the User. If this value is not passed, the default invite redirect URL that you set in your
    /// Dashboard is used. If you have not set a default sign-up redirect URL, an error is returned.
    pub invite_magic_link_url: std::option::Option<String>,
    /// invite_expiration_minutes: Set the expiration for the email magic link, in minutes. By default, it
    /// expires in 1 hour. The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub invite_expiration_minutes: std::option::Option<i32>,
    /// locale: Used to determine which language to use when sending the user this delivery method. Parameter is
    /// a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/), e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<InviteRequestLocale>,
}
/// InviteResponse: Response type for `Email.invite`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteResponse {
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
/// LoginOrCreateRequest: Request type for `Email.login_or_create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginOrCreateRequest {
    /// email: The email address of the end user.
    pub email: String,
    /// login_magic_link_url: The URL the end user clicks from the login Email Magic Link. This should be a URL
    /// that your app receives and parses and subsequently send an API request to authenticate the Magic Link
    /// and log in the User. If this value is not passed, the default login redirect URL that you set in your
    /// Dashboard is used. If you have not set a default login redirect URL, an error is returned.
    pub login_magic_link_url: std::option::Option<String>,
    /// signup_magic_link_url: The URL the end user clicks from the sign-up Email Magic Link. This should be a
    /// URL that your app receives and parses and subsequently send an API request to authenticate the Magic
    /// Link and sign-up the User. If this value is not passed, the default sign-up redirect URL that you set in
    /// your Dashboard is used. If you have not set a default sign-up redirect URL, an error is returned.
    pub signup_magic_link_url: std::option::Option<String>,
    /// login_expiration_minutes: Set the expiration for the login email magic link, in minutes. By default, it
    /// expires in 1 hour. The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub login_expiration_minutes: std::option::Option<i32>,
    /// signup_expiration_minutes: Set the expiration for the sign-up email magic link, in minutes. By default,
    /// it expires in 1 week. The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub signup_expiration_minutes: std::option::Option<i32>,
    /// login_template_id: Use a custom template for login emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Login.
    pub login_template_id: std::option::Option<String>,
    /// signup_template_id: Use a custom template for sign-up emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Sign-up.
    pub signup_template_id: std::option::Option<String>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// create_user_as_pending: Flag for whether or not to save a user as pending vs active in Stytch. Defaults
    /// to false.
    /// If true, users will be saved with status pending in Stytch's backend until authenticated.
    /// If false, users will be created as active. An example usage of
    /// a true flag would be to require users to verify their phone by entering the OTP code before creating
    /// an account for them.
    pub create_user_as_pending: std::option::Option<bool>,
    /// code_challenge: A base64url encoded SHA256 hash of a one time secret used to validate that the request
    /// starts and ends on the same device.
    pub code_challenge: std::option::Option<String>,
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
/// RevokeInviteRequest: Request type for `Email.revoke_invite`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevokeInviteRequest {
    /// email: The email of the user.
    pub email: String,
}
/// RevokeInviteResponse: Response type for `Email.revoke_invite`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeInviteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// SendRequest: Request type for `Email.send`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SendRequest {
    /// email: The email address of the User to send the Magic Link to.
    pub email: String,
    /// login_template_id: Use a custom template for login emails. By default, it will use your default email
    /// template. The template must be a template using our built-in customizations or a custom HTML email for
    /// Magic links - Login.
    pub login_template_id: std::option::Option<String>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// login_magic_link_url: The URL the end user clicks from the login Email Magic Link. This should be a URL
    /// that your app receives and parses and subsequently send an API request to authenticate the Magic Link
    /// and log in the User. If this value is not passed, the default login redirect URL that you set in your
    /// Dashboard is used. If you have not set a default login redirect URL, an error is returned.
    pub login_magic_link_url: std::option::Option<String>,
    /// signup_magic_link_url: The URL the end user clicks from the sign-up Email Magic Link. This should be a
    /// URL that your app receives and parses and subsequently send an API request to authenticate the Magic
    /// Link and sign-up the User. If this value is not passed, the default sign-up redirect URL that you set in
    /// your Dashboard is used. If you have not set a default sign-up redirect URL, an error is returned.
    pub signup_magic_link_url: std::option::Option<String>,
    /// login_expiration_minutes: Set the expiration for the login email magic link, in minutes. By default, it
    /// expires in 1 hour. The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub login_expiration_minutes: std::option::Option<i32>,
    /// signup_expiration_minutes: Set the expiration for the sign-up email magic link, in minutes. By default,
    /// it expires in 1 week. The minimum expiration is 5 minutes and the maximum is 7 days (10080 mins).
    pub signup_expiration_minutes: std::option::Option<i32>,
    /// code_challenge: A base64url encoded SHA256 hash of a one time secret used to validate that the request
    /// starts and ends on the same device.
    pub code_challenge: std::option::Option<String>,
    /// user_id: The unique ID of a specific User. You may use an external_id here if one is set for the user.
    pub user_id: std::option::Option<String>,
    /// session_token: The `session_token` of the user to associate the email with.
    pub session_token: std::option::Option<String>,
    /// session_jwt: The `session_jwt` of the user to associate the email with.
    pub session_jwt: std::option::Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum InviteRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum LoginOrCreateRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum SendRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}

pub struct Email {
    http_client: crate::client::Client,
}

impl Email {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn send(&self, body: SendRequest) -> crate::Result<SendResponse> {
        let path = String::from("/v1/magic_links/email/send");
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
        let path = String::from("/v1/magic_links/email/login_or_create");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn invite(&self, body: InviteRequest) -> crate::Result<InviteResponse> {
        let path = String::from("/v1/magic_links/email/invite");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn revoke_invite(
        &self,
        body: RevokeInviteRequest,
    ) -> crate::Result<RevokeInviteResponse> {
        let path = String::from("/v1/magic_links/email/revoke_invite");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
