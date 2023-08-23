# Stytch Rust Library

The Stytch Rust library makes it easy to use the Stytch user infrastructure API in Rust applications.

It pairs well with the Stytch [Web SDK](https://www.npmjs.com/package/@stytch/stytch-js) or your own custom authentication flow.

## Install
Add the following to `Cargo.toml`:
```
stytch = "0.1.0"
```
Then run `cargo build`

## Usage

You can find your API credentials in the [Stytch Dashboard](https://stytch.com/dashboard/api-keys).

This client library supports all of Stytch's live products:

**B2C**

- [x] [Email Magic Links](https://stytch.com/docs/api/send-by-email)
- [x] [Embeddable Magic Links](https://stytch.com/docs/api/create-magic-link)
- [x] [OAuth logins](https://stytch.com/docs/api/oauth-google-start)
- [x] [SMS passcodes](https://stytch.com/docs/api/send-otp-by-sms)
- [x] [WhatsApp passcodes](https://stytch.com/docs/api/whatsapp-send)
- [x] [Email passcodes](https://stytch.com/docs/api/send-otp-by-email)
- [x] [Session Management](https://stytch.com/docs/api/session-auth)
- [x] [WebAuthn](https://stytch.com/docs/api/webauthn-register-start)
- [x] [User Management](https://stytch.com/docs/api/create-user)
- [x] [Time-based one-time passcodes (TOTPs)](https://stytch.com/docs/api/totp-create)
- [x] [Crypto wallets](https://stytch.com/docs/api/crypto-wallet-authenticate-start)
- [x] [Passwords](https://stytch.com/docs/api/password-create)

**B2B**

- [x] [Organizations](https://stytch.com/docs/b2b/api/organization-object)
- [x] [Members](https://stytch.com/docs/b2b/api/member-object)
- [x] [Email Magic Links](https://stytch.com/docs/b2b/api/send-login-signup-email)
- [x] [OAuth logins](https://stytch.com/docs/b2b/api/oauth-google-start)
- [x] [Session Management](https://stytch.com/docs/b2b/api/session-object)
- [x] [Single-Sign On](https://stytch.com/docs/b2b/api/sso-authenticate-start)
- [x] [Discovery](https://stytch.com/docs/b2b/api/discovered-organization-object)
- [x] [Passwords](https://stytch.com/docs/b2b/api/passwords-authenticate)

**Shared**

- [x] [M2M](https://stytch.com/docs/api/m2m-client)

### Example B2C usage

Create an API client:

```rust
use stytch::consumer::{
    client::Client
};

let stytch_client = Client::new(
    "project-live-c60c0abe-c25a-4472-a9ed-320c6667d317",
    "secret-live-80JASucyk7z_G8Z-7dVwZVGXL5NT_qGAQ2I=",
).unwrap();
```

Send a magic link by email:

```rust
let resp = stytch_client
    .client
    .magic_links
    .email
    .login_or_create(LoginOrCreateRequest {
        email: "sandbox@stytch.com",
        ..Default::default()
    })
    .await;
```

Authenticate the token from the magic link:

```rust
let resp = stytch_client
    .client
    .magic_links
    .authenticate(AuthenticateRequest {
        token: "DOYoip3rvIMMW5lgItikFK-Ak1CfMsgjuiCyI7uuU94=",
        ..Default::default()
    })
    .await;
```

### Example B2B usage

Create an API client:

```rust
use stytch::b2b::{
    client::Client
};

let stytch_client = Client::new(
    "project-live-c60c0abe-c25a-4472-a9ed-320c6667d317",
    "secret-live-80JASucyk7z_G8Z-7dVwZVGXL5NT_qGAQ2I=",
).unwrap()
```

Create an organization

```rust
let allowed_domains = vec!["acme.co".to_string()];
let resp = stytch_client
    .client
    .organizations
    .create(CreateRequest {
        organization_name: "Acme Co",
        organization_slug: "acme-co",
        email_allowed_domains: allowed_domains,
        ..Default::default()
    })
    .await;
```

Log the first user into the organization

```rust
let resp = stytch_client
    .client
    .magic_links
    .email
    .login_or_signup(LoginOrSignupRequest {
        organization_id: "organization-id-from-create-response-..."
        email_address: "admin@acme.co"
        ..Default::default()
    })
    .await;
```

## Documentation

See example requests and responses for all the endpoints in the [Stytch API Reference](https://stytch.com/docs/api).

Follow one of the [integration guides](https://stytch.com/docs/guides) or start with one of our [example apps](https://stytch.com/docs/example-apps).

## Support

If you've found a bug, [open an issue](https://github.com/stytchauth/stytch-node/issues/new)!

If you have questions or want help troubleshooting, join us in [Slack](https://join.slack.com/t/stytch/shared_invite/zt-nil4wo92-jApJ9Cl32cJbEd9esKkvyg) or email support@stytch.com.

If you've found a security vulnerability, please follow our [responsible disclosure instructions](https://stytch.com/docs/resources/security-and-trust/security#:~:text=Responsible%20disclosure%20program).

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md)

## Code of Conduct

Everyone interacting in the Stytch project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](CODE_OF_CONDUCT.md).
