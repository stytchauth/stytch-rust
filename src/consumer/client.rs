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
    pub fn new(project_id: String, secret: String) -> crate::Result<Self> {
        Ok(Client::new_with_http_client(crate::reqwest::Client::new(
            project_id, secret,
        )?))
    }

    pub fn new_with_http_client(http_client: crate::reqwest::Client) -> Self {
        Client {
            crypto_wallets: CryptoWallets::new(http_client.clone()),
            m2m: M2M::new(http_client.clone()),
            magic_links: MagicLinks::new(http_client.clone()),
            oauth: OAuth::new(http_client.clone()),
            otps: OTPs::new(http_client.clone()),
            passwords: Passwords::new(http_client.clone()),
            sessions: Sessions::new(http_client.clone()),
            totps: TOTPs::new(http_client.clone()),
            users: Users::new(http_client.clone()),
            webauthn: WebAuthn::new(http_client.clone()),
        }
    }
}