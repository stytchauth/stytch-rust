// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::attribute::Attributes;
use crate::consumer::users::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmazonOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppleOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// AuthenticationFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticationFactor {
    /// type_: The type of authentication factor. The possible values are: `magic_link`, `otp`,
    ///    `oauth`, `password`, `email_otp`, or `sso` .
    #[serde(rename = "type")]
    pub type_: AuthenticationFactorType,
    /// delivery_method: The method that was used to deliver the authentication factor. The possible values
    /// depend on the `type`:
    ///
    ///   `magic_link` – Only `email`.
    ///
    ///   `otp` –  Either `sms` or `email` .
    ///
    ///   `oauth` – Either `oauth_google` or `oauth_microsoft`.
    ///
    ///   `password` – Only `knowledge`.
    ///
    ///   `sso` – Either `sso_saml` or `sso_oidc`.
    ///
    pub delivery_method: AuthenticationFactorDeliveryMethod,
    /// last_authenticated_at: The timestamp when the factor was last authenticated.
    pub last_authenticated_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// created_at: The timestamp when the factor was initially authenticated.
    pub created_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// updated_at: The timestamp when the factor was last updated.
    pub updated_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// email_factor: Information about the email factor, if one is present.
    pub email_factor: std::option::Option<EmailFactor>,
    /// phone_number_factor: Information about the phone number factor, if one is present.
    pub phone_number_factor: std::option::Option<PhoneNumberFactor>,
    /// google_oauth_factor: Information about the Google OAuth factor, if one is present.
    pub google_oauth_factor: std::option::Option<GoogleOAuthFactor>,
    /// microsoft_oauth_factor: Information about the Microsoft OAuth factor, if one is present.
    pub microsoft_oauth_factor: std::option::Option<MicrosoftOAuthFactor>,
    pub apple_oauth_factor: std::option::Option<AppleOAuthFactor>,
    pub webauthn_factor: std::option::Option<WebAuthnFactor>,
    /// authenticator_app_factor: Information about the TOTP-backed Authenticator App factor, if one is present.
    pub authenticator_app_factor: std::option::Option<AuthenticatorAppFactor>,
    pub github_oauth_factor: std::option::Option<GithubOAuthFactor>,
    pub recovery_code_factor: std::option::Option<RecoveryCodeFactor>,
    pub facebook_oauth_factor: std::option::Option<FacebookOAuthFactor>,
    pub crypto_wallet_factor: std::option::Option<CryptoWalletFactor>,
    pub amazon_oauth_factor: std::option::Option<AmazonOAuthFactor>,
    pub bitbucket_oauth_factor: std::option::Option<BitbucketOAuthFactor>,
    pub coinbase_oauth_factor: std::option::Option<CoinbaseOAuthFactor>,
    pub discord_oauth_factor: std::option::Option<DiscordOAuthFactor>,
    pub figma_oauth_factor: std::option::Option<FigmaOAuthFactor>,
    pub git_lab_oauth_factor: std::option::Option<GitLabOAuthFactor>,
    pub instagram_oauth_factor: std::option::Option<InstagramOAuthFactor>,
    pub linked_in_oauth_factor: std::option::Option<LinkedInOAuthFactor>,
    pub shopify_oauth_factor: std::option::Option<ShopifyOAuthFactor>,
    pub slack_oauth_factor: std::option::Option<SlackOAuthFactor>,
    pub snapchat_oauth_factor: std::option::Option<SnapchatOAuthFactor>,
    pub spotify_oauth_factor: std::option::Option<SpotifyOAuthFactor>,
    pub steam_oauth_factor: std::option::Option<SteamOAuthFactor>,
    pub tik_tok_oauth_factor: std::option::Option<TikTokOAuthFactor>,
    pub twitch_oauth_factor: std::option::Option<TwitchOAuthFactor>,
    pub twitter_oauth_factor: std::option::Option<TwitterOAuthFactor>,
    pub embeddable_magic_link_factor: std::option::Option<EmbeddableMagicLinkFactor>,
    pub biometric_factor: std::option::Option<BiometricFactor>,
    /// saml_sso_factor: Information about the SAML SSO factor, if one is present.
    pub saml_sso_factor: std::option::Option<SAMLSSOFactor>,
    /// oidc_sso_factor: Information about the OIDC SSO factor, if one is present.
    pub oidc_sso_factor: std::option::Option<OIDCSSOFactor>,
    pub salesforce_oauth_factor: std::option::Option<SalesforceOAuthFactor>,
    pub yahoo_oauth_factor: std::option::Option<YahooOAuthFactor>,
    pub hubspot_oauth_factor: std::option::Option<HubspotOAuthFactor>,
    pub slack_oauth_exchange_factor: std::option::Option<SlackOAuthExchangeFactor>,
    pub hubspot_oauth_exchange_factor: std::option::Option<HubspotOAuthExchangeFactor>,
    pub github_oauth_exchange_factor: std::option::Option<GithubOAuthExchangeFactor>,
    pub google_oauth_exchange_factor: std::option::Option<GoogleOAuthExchangeFactor>,
    /// impersonated_factor: Information about the impersonated factor, if one is present.
    pub impersonated_factor: std::option::Option<ImpersonatedFactor>,
}
/// AuthenticatorAppFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticatorAppFactor {
    /// totp_id: Globally unique UUID that identifies a TOTP instance.
    pub totp_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiometricFactor {
    pub biometric_registration_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitbucketOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinbaseOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoWalletFactor {
    pub crypto_wallet_id: String,
    pub crypto_wallet_address: String,
    pub crypto_wallet_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// EmailFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailFactor {
    /// email_id: The globally unique UUID of the Member's email.
    pub email_id: String,
    /// email_address: The email address of the Member.
    pub email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddableMagicLinkFactor {
    pub embedded_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacebookOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FigmaOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitLabOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubOAuthExchangeFactor {
    pub email_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleOAuthExchangeFactor {
    pub email_id: String,
}
/// GoogleOAuthFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleOAuthFactor {
    /// id: The unique ID of an OAuth registration.
    pub id: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the `sub` or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// email_id: The globally unique UUID of the Member's email.
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HubspotOAuthExchangeFactor {
    pub email_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HubspotOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// ImpersonatedFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImpersonatedFactor {
    /// impersonator_id: The unique UUID of the impersonator. For impersonation sessions initiated via the
    /// Stytch dashboard, the `impersonator_id` will be the impersonator's Stytch workspace id.
    pub impersonator_id: String,
    /// impersonator_email_address: The email address of the impersonator.
    pub impersonator_email_address: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWK {
    pub kty: String,
    #[serde(rename = "use")]
    pub use_: String,
    pub key_ops: std::vec::Vec<String>,
    pub alg: String,
    pub kid: String,
    pub x5c: std::vec::Vec<String>,
    pub x5tS256: String,
    pub n: String,
    pub e: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedInOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// MicrosoftOAuthFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MicrosoftOAuthFactor {
    /// id: The unique ID of an OAuth registration.
    pub id: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the `sub` or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// email_id: The globally unique UUID of the Member's email.
    pub email_id: std::option::Option<String>,
}
/// OIDCSSOFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OIDCSSOFactor {
    /// id: The unique ID of an SSO Registration.
    pub id: String,
    /// provider_id: Globally unique UUID that identifies a specific OIDC Connection.
    pub provider_id: String,
    /// external_id: The ID of the member given by the identity provider.
    pub external_id: String,
}
/// PhoneNumberFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneNumberFactor {
    /// phone_id: The globally unique UUID of the Member's phone number.
    pub phone_id: String,
    /// phone_number: The phone number of the Member.
    pub phone_number: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryCodeFactor {
    pub totp_recovery_code_id: String,
}
/// SAMLSSOFactor:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SAMLSSOFactor {
    /// id: The unique ID of an SSO Registration.
    pub id: String,
    /// provider_id: Globally unique UUID that identifies a specific SAML Connection.
    pub provider_id: String,
    /// external_id: The ID of the member given by the identity provider.
    pub external_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalesforceOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// Session:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    /// session_id: A unique identifier for a specific Session.
    pub session_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// authentication_factors: An array of different authentication factors that comprise a Session.
    pub authentication_factors: std::vec::Vec<AuthenticationFactor>,
    /// started_at: The timestamp when the Session was created. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub started_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// last_accessed_at: The timestamp when the Session was last accessed. Values conform to the RFC 3339
    /// standard and are expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub last_accessed_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// expires_at: The timestamp when the Session expires. Values conform to the RFC 3339 standard and are
    /// expressed in UTC, e.g. `2021-12-29T12:33:09Z`.
    pub expires_at: std::option::Option<chrono::DateTime<chrono::Utc>>,
    /// attributes: Provided attributes help with fraud detection.
    pub attributes: std::option::Option<Attributes>,
    /// custom_claims: The custom claims map for a Session. Claims can be added to a session during a Sessions
    /// authenticate call.
    pub custom_claims: std::option::Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShopifyOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlackOAuthExchangeFactor {
    pub email_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlackOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapchatOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TikTokOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitchOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitterOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebAuthnFactor {
    pub webauthn_registration_id: String,
    pub domain: String,
    pub user_agent: std::option::Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YahooOAuthFactor {
    pub id: String,
    pub provider_subject: String,
    pub email_id: std::option::Option<String>,
}
/// AuthenticateRequest: Request type for `Sessions.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// session_token: The session token to authenticate.
    pub session_token: std::option::Option<String>,
    /// session_duration_minutes: Set the session lifetime to be this many minutes from now; minimum of 5 and a
    /// maximum of 527040 minutes (366 days). Note that a successful authentication will continue to extend the
    /// session this many minutes.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_jwt: The JWT to authenticate. You may provide a JWT that has expired according to its `exp`
    /// claim and needs to be refreshed. If the signature is valid and the underlying session is still active
    /// then Stytch will return a new JWT.
    pub session_jwt: std::option::Option<String>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in `session_duration_minutes`. Claims will be
    /// included on the Session object and in the JWT. To update a key in an existing Session, supply a new
    /// value. To delete a key, supply a null value.
    ///
    ///   Custom claims made with reserved claims ("iss", "sub", "aud", "exp", "nbf", "iat", "jti") will be
    /// ignored. Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
}
/// AuthenticateResponse: Response type for `Sessions.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// session: If you initiate a Session, by including `session_duration_minutes` in your authenticate call,
    /// you'll receive a full Session object in the response.
    ///
    ///   See [GET sessions](https://stytch.com/docs/api/session-get) for complete response fields.
    ///
    pub session: Session,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// GetJWKSRequest: Request type for `Sessions.get_jwks`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetJWKSRequest {
    /// project_id: The `project_id` to get the JWKS for.
    pub project_id: String,
}
/// GetJWKSResponse: Response type for `Sessions.get_jwks`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetJWKSResponse {
    /// keys: The list of JWKs associated with the project.
    pub keys: std::vec::Vec<JWK>,
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
/// GetRequest: Request type for `Sessions.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// user_id: The `user_id` to get active Sessions for.
    pub user_id: String,
}
/// GetResponse: Response type for `Sessions.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// sessions: An array of Session objects.
    pub sessions: std::vec::Vec<Session>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// MigrateRequest: Request type for `Sessions.migrate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MigrateRequest {
    /// session_token: The authorization token Stytch will pass in to the external userinfo endpoint.
    pub session_token: String,
    /// session_duration_minutes: Set the session lifetime to be this many minutes from now. This will start a
    /// new session if one doesn't already exist,
    ///   returning both an opaque `session_token` and `session_jwt` for this session. Remember that the
    /// `session_jwt` will have a fixed lifetime of
    ///   five minutes regardless of the underlying session duration, and will need to be refreshed over time.
    ///
    ///   This value must be a minimum of 5 and a maximum of 527040 minutes (366 days).
    ///
    ///   If a `session_token` or `session_jwt` is provided then a successful authentication will continue to
    /// extend the session this many minutes.
    ///
    ///   If the `session_duration_minutes` parameter is not specified, a Stytch session will not be created.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in `session_duration_minutes`. Claims will be
    /// included on the Session object and in the JWT. To update a key in an existing Session, supply a new
    /// value. To delete a key, supply a null value.
    ///
    ///   Custom claims made with reserved claims ("iss", "sub", "aud", "exp", "nbf", "iat", "jti") will be
    /// ignored. Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
}
/// MigrateResponse: Response type for `Sessions.migrate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MigrateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// user_id: The unique ID of the affected User.
    pub user_id: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// user: The `user` object affected by this API call. See the
    /// [Get user endpoint](https://stytch.com/docs/api/get-user) for complete response field details.
    pub user: User,
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// session: If you initiate a Session, by including `session_duration_minutes` in your authenticate call,
    /// you'll receive a full Session object in the response.
    ///
    ///   See [GET sessions](https://stytch.com/docs/api/session-get) for complete response fields.
    ///
    pub session: std::option::Option<Session>,
}
/// RevokeRequest: Request type for `Sessions.revoke`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevokeRequest {
    /// session_id: The `session_id` to revoke.
    pub session_id: std::option::Option<String>,
    /// session_token: The session token to revoke.
    pub session_token: std::option::Option<String>,
    /// session_jwt: A JWT for the session to revoke.
    pub session_jwt: std::option::Option<String>,
}
/// RevokeResponse: Response type for `Sessions.revoke`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeResponse {
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AuthenticationFactorDeliveryMethod {
    #[serde(rename = "email")]
    #[default]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "embedded")]
    Embedded,
    #[serde(rename = "oauth_google")]
    OAuthGoogle,
    #[serde(rename = "oauth_microsoft")]
    OAuthMicrosoft,
    #[serde(rename = "oauth_apple")]
    OAuthApple,
    #[serde(rename = "webauthn_registration")]
    WebAuthnRegistration,
    #[serde(rename = "authenticator_app")]
    AuthenticatorApp,
    #[serde(rename = "oauth_github")]
    OAuthGithub,
    #[serde(rename = "recovery_code")]
    RecoveryCode,
    #[serde(rename = "oauth_facebook")]
    OAuthFacebook,
    #[serde(rename = "crypto_wallet")]
    CryptoWallet,
    #[serde(rename = "oauth_amazon")]
    OAuthAmazon,
    #[serde(rename = "oauth_bitbucket")]
    OAuthBitbucket,
    #[serde(rename = "oauth_coinbase")]
    OAuthCoinbase,
    #[serde(rename = "oauth_discord")]
    OAuthDiscord,
    #[serde(rename = "oauth_figma")]
    OAuthFigma,
    #[serde(rename = "oauth_gitlab")]
    OAuthGitlab,
    #[serde(rename = "oauth_instagram")]
    OAuthInstagram,
    #[serde(rename = "oauth_linkedin")]
    OAuthLinkedin,
    #[serde(rename = "oauth_shopify")]
    OAuthShopify,
    #[serde(rename = "oauth_slack")]
    OAuthSlack,
    #[serde(rename = "oauth_snapchat")]
    OAuthSnapchat,
    #[serde(rename = "oauth_spotify")]
    OAuthSpotify,
    #[serde(rename = "oauth_steam")]
    OAuthSteam,
    #[serde(rename = "oauth_tiktok")]
    OAuthTiktok,
    #[serde(rename = "oauth_twitch")]
    OAuthTwitch,
    #[serde(rename = "oauth_twitter")]
    OAuthTwitter,
    #[serde(rename = "knowledge")]
    Knowledge,
    #[serde(rename = "biometric")]
    Biometric,
    #[serde(rename = "sso_saml")]
    SSOSAML,
    #[serde(rename = "sso_oidc")]
    SSOOIDC,
    #[serde(rename = "oauth_salesforce")]
    OAuthSalesforce,
    #[serde(rename = "oauth_yahoo")]
    OAuthYahoo,
    #[serde(rename = "oauth_hubspot")]
    OAuthHubspot,
    #[serde(rename = "imported_auth0")]
    ImportedAuth0,
    #[serde(rename = "oauth_exchange_slack")]
    OAuthExchangeSlack,
    #[serde(rename = "oauth_exchange_hubspot")]
    OAuthExchangeHubspot,
    #[serde(rename = "oauth_exchange_github")]
    OAuthExchangeGithub,
    #[serde(rename = "oauth_exchange_google")]
    OAuthExchangeGoogle,
    #[serde(rename = "impersonation")]
    Impersonation,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AuthenticationFactorType {
    #[serde(rename = "magic_link")]
    #[default]
    MagicLink,
    #[serde(rename = "otp")]
    OTP,
    #[serde(rename = "oauth")]
    OAuth,
    #[serde(rename = "webauthn")]
    WebAuthn,
    #[serde(rename = "totp")]
    TOTP,
    #[serde(rename = "crypto")]
    Crypto,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "signature_challenge")]
    SignatureChallenge,
    #[serde(rename = "sso")]
    SSO,
    #[serde(rename = "imported")]
    Imported,
    #[serde(rename = "recovery_codes")]
    RecoveryCodes,
    #[serde(rename = "email_otp")]
    EmailOTP,
    #[serde(rename = "impersonated")]
    Impersonated,
}

pub struct Sessions {
    http_client: crate::client::Client,
}

impl Sessions {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }

    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let path = String::from("/v1/sessions");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/sessions/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn revoke(&self, body: RevokeRequest) -> crate::Result<RevokeResponse> {
        let path = String::from("/v1/sessions/revoke");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn migrate(&self, body: MigrateRequest) -> crate::Result<MigrateResponse> {
        let path = String::from("/v1/sessions/migrate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn get_jwks(&self, body: GetJWKSRequest) -> crate::Result<GetJWKSResponse> {
        let project_id = &body.project_id;
        let path = format!("/v1/sessions/jwks/{project_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
}
