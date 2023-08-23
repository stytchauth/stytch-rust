// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use serde::{Deserialize, Serialize};

/// BiometricRegistration:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiometricRegistration {
    /// biometric_registration_id: The unique ID for a biometric registration.
    pub biometric_registration_id: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
}

/// CryptoWallet:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoWallet {
    /// crypto_wallet_id: The unique ID for a crypto wallet
    pub crypto_wallet_id: String,
    /// crypto_wallet_address: The actual blockchain address of the User's crypto wallet.
    pub crypto_wallet_address: String,
    /// crypto_wallet_type: The blockchain that the User's crypto wallet operates on, e.g. Ethereum, Solana, etc.
    pub crypto_wallet_type: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
}

/// Email:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email {
    /// email_id: The unique ID of a specific email address.
    pub email_id: String,
    /// email: The email address.
    pub email: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
}

/// Name:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    /// first_name: The first name of the user.
    pub first_name: std::option::Option<String>,
    /// middle_name: The middle name(s) of the user.
    pub middle_name: std::option::Option<String>,
    /// last_name: The last name of the user.
    pub last_name: std::option::Option<String>,
}

/// OAuthProvider:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthProvider {
    /// provider_type: Denotes the OAuth identity provider that the user has authenticated with, e.g. Google,
    /// Facebook, GitHub etc.
    pub provider_type: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the "sub" or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// profile_picture_url: If available, the `profile_picture_url` is a url of the User's profile picture set
    /// in OAuth identity the provider that the User has authenticated with, e.g. Facebook profile picture.
    pub profile_picture_url: String,
    /// locale: If available, the `locale` is the User's locale set in the OAuth identity provider that the user
    /// has authenticated with.
    pub locale: String,
    /// oauth_user_registration_id: The unique ID for an OAuth registration.
    pub oauth_user_registration_id: String,
}

/// Password:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Password {
    /// password_id: The unique ID of a specific password
    pub password_id: String,
    /// requires_reset: Indicates whether this password requires a password reset
    pub requires_reset: bool,
}

/// PhoneNumber:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneNumber {
    /// phone_id: The unique ID for the phone number.
    pub phone_id: String,
    /// phone_number: The phone number.
    pub phone_number: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
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

/// SearchUsersQuery:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchUsersQuery {
    /// operator: The action to perform on the operands. The accepted value are:
    ///
    ///   `AND` – all the operand values provided must match.
    ///   
    ///   `OR` – the operator will return any matches to at least one of the operand values you supply.
    pub operator: SearchUsersQueryOperator,
    /// operands: An array of operand objects that contains all of the filters and values to apply to your
    /// search search query.
    pub operands: std::vec::Vec<serde_json::Value>,
}

/// TOTP:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOTP {
    /// totp_id: The unique ID for a TOTP instance.
    pub totp_id: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
}

/// User:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// emails: An array of email objects for the User.
    pub emails: std::vec::Vec<Email>,
    /// status: The status of the User. The possible values are `pending` and `active`.
    pub status: String,
    /// phone_numbers: An array of phone number objects linked to the User.
    pub phone_numbers: std::vec::Vec<PhoneNumber>,
    /// webauthn_registrations: An array that contains a list of all WebAuthn registrations for a given User in
    /// the Stytch API.
    pub webauthn_registrations: std::vec::Vec<WebAuthnRegistration>,
    /// providers: An array of OAuth `provider` objects linked to the User.
    pub providers: std::vec::Vec<OAuthProvider>,
    /// totps: An array containing a list of all TOTP instances for a given User in the Stytch API.
    pub totps: std::vec::Vec<TOTP>,
    /// crypto_wallets: An array contains a list of all crypto wallets for a given User in the Stytch API.
    pub crypto_wallets: std::vec::Vec<CryptoWallet>,
    /// biometric_registrations: An array that contains a list of all biometric registrations for a given User
    /// in the Stytch API.
    pub biometric_registrations: std::vec::Vec<BiometricRegistration>,
    /// name: The name of the User. Each field in the `name` object is optional.
    pub name: std::option::Option<Name>,
    /// created_at: The timestamp of the User's creation. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub created_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// password: The password object is returned for users with a password.
    pub password: std::option::Option<Password>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: The `untrusted_metadata` field contains an arbitrary JSON object of
    /// application-specific data. Untrusted metadata can be edited by end users directly via the SDK, and
    /// **cannot be used to store critical information.** See the
    /// [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
}

/// WebAuthnRegistration:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAuthnRegistration {
    /// webauthn_registration_id: The unique ID for the WebAuthn registration.
    pub webauthn_registration_id: String,
    /// domain: The `domain` on which a WebAuthn registration was started. This will be the domain of your app.
    pub domain: String,
    /// user_agent: The user agent of the User.
    pub user_agent: String,
    /// verified: The verified boolean denotes whether or not this send method, e.g. phone number, email
    /// address, etc., has been successfully authenticated by the User.
    pub verified: bool,
    /// authenticator_type: The `authenticator_type` string displays the requested authenticator type of the
    /// WebAuthn device. The two valid types are "platform" and "cross-platform". If no value is present, the
    /// WebAuthn device was created without an authenticator type preference.
    pub authenticator_type: String,
}

/// CreateRequest: Request type for `Users.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// email: The email address of the end user.
    pub email: std::option::Option<String>,
    /// name: The name of the user. Each field in the name object is optional.
    pub name: std::option::Option<Name>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// phone_number: The phone number to use for one-time passcodes. The phone number should be in E.164
    /// format. The phone number should be in E.164 format (i.e. +1XXXXXXXXXX). You may use +10000000000 to test
    /// this endpoint, see [Testing](https://stytch.com/docs/home#resources_testing) for more detail.
    pub phone_number: std::option::Option<String>,
    /// create_user_as_pending: Flag for whether or not to save a user as pending vs active in Stytch. Defaults
    /// to false.
    ///         If true, users will be saved with status pending in Stytch's backend until authenticated.
    ///         If false, users will be created as active. An example usage of
    ///         a true flag would be to require users to verify their phone by entering the OTP code before
    /// creating
    ///         an account for them.
    pub create_user_as_pending: std::option::Option<bool>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: The `untrusted_metadata` field contains an arbitrary JSON object of
    /// application-specific data. Untrusted metadata can be edited by end users directly via the SDK, and
    /// **cannot be used to store critical information.** See the
    /// [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
}

/// CreateResponse: Response type for `Users.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// email_id: The unique ID of a specific email address.
    pub email_id: String,
    /// status: The status of the User. The possible values are `pending` and `active`.
    pub status: String,
    /// phone_id: The unique ID for the phone number.
    pub phone_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteBiometricRegistrationRequest: Request type for `Users.delete_biometric_registration`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteBiometricRegistrationRequest {
    /// biometric_registration_id: The `biometric_registration_id` to be deleted.
    pub biometric_registration_id: String,
}

/// DeleteBiometricRegistrationResponse: Response type for `Users.delete_biometric_registration`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteBiometricRegistrationResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteCryptoWalletRequest: Request type for `Users.delete_crypto_wallet`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteCryptoWalletRequest {
    /// crypto_wallet_id: The `crypto_wallet_id` to be deleted.
    pub crypto_wallet_id: String,
}

/// DeleteCryptoWalletResponse: Response type for `Users.delete_crypto_wallet`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCryptoWalletResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteEmailRequest: Request type for `Users.delete_email`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteEmailRequest {
    /// email_id: The `email_id` to be deleted.
    pub email_id: String,
}

/// DeleteEmailResponse: Response type for `Users.delete_email`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteEmailResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteOAuthRegistrationRequest: Request type for `Users.delete_oauth_registration`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteOAuthRegistrationRequest {
    /// oauth_user_registration_id: The `oauth_user_registration_id` to be deleted.
    pub oauth_user_registration_id: String,
}

/// DeleteOAuthRegistrationResponse: Response type for `Users.delete_oauth_registration`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteOAuthRegistrationResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeletePasswordRequest: Request type for `Users.delete_password`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeletePasswordRequest {
    /// password_id: The `password_id` to be deleted.
    pub password_id: String,
}

/// DeletePasswordResponse: Response type for `Users.delete_password`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePasswordResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeletePhoneNumberRequest: Request type for `Users.delete_phone_number`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeletePhoneNumberRequest {
    /// phone_id: The `phone_id` to be deleted.
    pub phone_id: String,
}

/// DeletePhoneNumberResponse: Response type for `Users.delete_phone_number`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePhoneNumberResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteRequest: Request type for `Users.delete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    /// user_id: The unique ID of a specific User.
    pub user_id: String,
}

/// DeleteResponse: Response type for `Users.delete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the deleted User.
    pub user_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteTOTPRequest: Request type for `Users.delete_totp`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteTOTPRequest {
    /// totp_id: The `totp_id` to be deleted.
    pub totp_id: String,
}

/// DeleteTOTPResponse: Response type for `Users.delete_totp`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteTOTPResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// DeleteWebAuthnRegistrationRequest: Request type for `Users.delete_webauthn_registration`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteWebAuthnRegistrationRequest {
    /// webauthn_registration_id: The `webauthn_registration_id` to be deleted.
    pub webauthn_registration_id: String,
}

/// DeleteWebAuthnRegistrationResponse: Response type for `Users.delete_webauthn_registration`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteWebAuthnRegistrationResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// GetRequest: Request type for `Users.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// user_id: The unique ID of a specific User.
    pub user_id: String,
}

/// GetResponse: Response type for `Users.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the returned User.
    pub user_id: String,
    /// emails: An array of email objects for the User.
    pub emails: std::vec::Vec<Email>,
    /// status: The status of the User. The possible values are `pending` and `active`.
    pub status: String,
    /// phone_numbers: An array of phone number objects linked to the User.
    pub phone_numbers: std::vec::Vec<PhoneNumber>,
    /// webauthn_registrations: An array that contains a list of all WebAuthn registrations for a given User in
    /// the Stytch API.
    pub webauthn_registrations: std::vec::Vec<WebAuthnRegistration>,
    /// providers: An array of OAuth `provider` objects linked to the User.
    pub providers: std::vec::Vec<OAuthProvider>,
    /// totps: An array containing a list of all TOTP instances for a given User in the Stytch API.
    pub totps: std::vec::Vec<TOTP>,
    /// crypto_wallets: An array contains a list of all crypto wallets for a given User in the Stytch API.
    pub crypto_wallets: std::vec::Vec<CryptoWallet>,
    /// biometric_registrations: An array that contains a list of all biometric registrations for a given User
    /// in the Stytch API.
    pub biometric_registrations: std::vec::Vec<BiometricRegistration>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// name: The name of the User. Each field in the `name` object is optional.
    pub name: std::option::Option<Name>,
    /// created_at: The timestamp of the User's creation. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub created_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// password: The password object is returned for users with a password.
    pub password: std::option::Option<Password>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: The `untrusted_metadata` field contains an arbitrary JSON object of
    /// application-specific data. Untrusted metadata can be edited by end users directly via the SDK, and
    /// **cannot be used to store critical information.** See the
    /// [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
}

/// SearchRequest: Request type for `Users.search`.
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
    pub limit: std::option::Option<i32>,
    /// query: The optional query object contains the operator, i.e. `AND` or `OR`, and the operands that will
    /// filter your results. Only an operator is required. If you include no operands, no filtering will be
    /// applied. If you include no query object, it will return all results with no filtering applied.
    pub query: std::option::Option<SearchUsersQuery>,
}

/// SearchResponse: Response type for `Users.search`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// results: An array of results that match your search query.
    pub results: std::vec::Vec<User>,
    /// results_metadata: The search `results_metadata` object contains metadata relevant to your specific query
    /// like total and `next_cursor`.
    pub results_metadata: ResultsMetadata,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

/// UpdateRequest: Request type for `Users.update`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    /// user_id: The unique ID of a specific User.
    pub user_id: String,
    /// name: The name of the user. Each field in the name object is optional.
    pub name: std::option::Option<Name>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: The `untrusted_metadata` field contains an arbitrary JSON object of
    /// application-specific data. Untrusted metadata can be edited by end users directly via the SDK, and
    /// **cannot be used to store critical information.** See the
    /// [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
}

/// UpdateResponse: Response type for `Users.update`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the updated User.
    pub user_id: String,
    /// emails: An array of email objects for the User.
    pub emails: std::vec::Vec<Email>,
    /// phone_numbers: An array of phone number objects linked to the User.
    pub phone_numbers: std::vec::Vec<PhoneNumber>,
    /// crypto_wallets: An array contains a list of all crypto wallets for a given User in the Stytch API.
    pub crypto_wallets: std::vec::Vec<CryptoWallet>,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum SearchUsersQueryOperator {
    #[serde(rename = "or")]
    #[default]
    OR,
    #[serde(rename = "and")]
    AND,
}

pub struct Users {
    http_client: crate::reqwest::Client,
}

impl Users {
    pub fn new(http_client: crate::reqwest::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let path = format!("/v1/users");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let user_id = &body.user_id;
        let path = format!("/v1/users/{user_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn search(&self, body: SearchRequest) -> crate::Result<SearchResponse> {
        let path = format!("/v1/users/search");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let user_id = &body.user_id;
        let path = format!("/v1/users/{user_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete(&self, body: DeleteRequest) -> crate::Result<DeleteResponse> {
        let user_id = &body.user_id;
        let path = format!("/v1/users/{user_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_email(
        &self,
        body: DeleteEmailRequest,
    ) -> crate::Result<DeleteEmailResponse> {
        let email_id = &body.email_id;
        let path = format!("/v1/users/emails/{email_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_phone_number(
        &self,
        body: DeletePhoneNumberRequest,
    ) -> crate::Result<DeletePhoneNumberResponse> {
        let phone_id = &body.phone_id;
        let path = format!("/v1/users/phone_numbers/{phone_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_webauthn_registration(
        &self,
        body: DeleteWebAuthnRegistrationRequest,
    ) -> crate::Result<DeleteWebAuthnRegistrationResponse> {
        let webauthn_registration_id = &body.webauthn_registration_id;
        let path = format!("/v1/users/webauthn_registrations/{webauthn_registration_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_biometric_registration(
        &self,
        body: DeleteBiometricRegistrationRequest,
    ) -> crate::Result<DeleteBiometricRegistrationResponse> {
        let biometric_registration_id = &body.biometric_registration_id;
        let path = format!("/v1/users/biometric_registrations/{biometric_registration_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_totp(&self, body: DeleteTOTPRequest) -> crate::Result<DeleteTOTPResponse> {
        let totp_id = &body.totp_id;
        let path = format!("/v1/users/totps/{totp_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_crypto_wallet(
        &self,
        body: DeleteCryptoWalletRequest,
    ) -> crate::Result<DeleteCryptoWalletResponse> {
        let crypto_wallet_id = &body.crypto_wallet_id;
        let path = format!("/v1/users/crypto_wallets/{crypto_wallet_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_password(
        &self,
        body: DeletePasswordRequest,
    ) -> crate::Result<DeletePasswordResponse> {
        let password_id = &body.password_id;
        let path = format!("/v1/users/passwords/{password_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn delete_oauth_registration(
        &self,
        body: DeleteOAuthRegistrationRequest,
    ) -> crate::Result<DeleteOAuthRegistrationResponse> {
        let oauth_user_registration_id = &body.oauth_user_registration_id;
        let path = format!("/v1/users/oauth/{oauth_user_registration_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
}
