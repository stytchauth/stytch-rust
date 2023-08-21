// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::crypto_wallets::CryptoWallets;
use crate::consumer::m2m::M2M;
use crate::consumer::magic_links::MagicLinks;
use crate::consumer::oauth::OAuth;
use crate::consumer::otp::OTPs;
use crate::consumer::passwords::Passwords;
use crate::consumer::sessions::Sessions;
use crate::consumer::totps::TOTPs;
use crate::consumer::users::Users;
use crate::consumer::webauthn::WebAuthn;

pub struct Client {
    pub crypto_wallets: CryptoWallets,
    pub m2m: M2M,
    pub magic_links: MagicLinks,
    pub oauth: OAuth,
    pub otps: OTPs,
    pub passwords: Passwords,
    pub sessions: Sessions,
    pub totps: TOTPs,
    pub users: Users,
    pub webauthn: WebAuthn,
}

impl Client {
    pub fn new(project_id: String, secret: String) -> Self {
        Client::NewWithHTTPClient(reqwest::Client::new())
    }

    pub fn new_with_http_client(http_client: HttpClient) -> Self {
        Client {
            project_id,
            secret,
            crypto_wallets: CryptoWallets::New(http_client),
            m2m: M2M::New(http_client),
            magic_links: MagicLinks::New(http_client),
            oauth: OAuth::New(http_client),
            otps: OTPs::New(http_client),
            passwords: Passwords::New(http_client),
            sessions: Sessions::New(http_client),
            totps: TOTPs::New(http_client),
            users: Users::New(http_client),
            webauthn: WebAuthn::New(http_client),
        }
    }
}
