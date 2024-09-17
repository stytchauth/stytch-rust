# Stytch Rust Library

The Stytch Rust library makes it easy to use the Stytch user infrastructure API in server-side Rust applications.

It pairs well with the Stytch [Web SDK](https://www.npmjs.com/package/@stytch/vanilla-js) or your own custom authentication flow.

The minimum supported Rust version (MSRV) of this library is Rust 1.70.

## Install

Use `cargo add stytch` to add this to your `Cargo.toml`:

```toml
stytch = "4.1.2"
```

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
let client = stytch::consumer::client::Client::new(
    String::from("project-live-c60c0abe-c25a-4472-a9ed-320c6667d317"),
    String::from("secret-live-80JASucyk7z_G8Z-7dVwZVGXL5NT_qGAQ2I="),
)?;
```

Send a magic link by email:

```rust
use stytch::consumer::magic_links_email::LoginOrCreateRequest;

let resp = client.magic_links.email.login_or_create(LoginOrCreateRequest{
    email: String::from("sandbox@stytch.com"),
    login_magic_link_url: Some(String::from("https://example.com/authenticate")),
    signup_magic_link_url: Some(String::from("https://example.com/authenticate")),
    ..Default::default()
}).await?;
```

Authenticate the token from the magic link:

```rust
use stytch::consumer::magic_links::AuthenticateRequest;

let resp = client.magic_links.authenticate(AuthenticateRequest {
    token: String::from("DOYoip3rvIMMW5lgItikFK-Ak1CfMsgjuiCyI7uuU94="),
    ..Default::default()
})
.await?;
```

### Example B2B usage

Create an API client:

```rust
let client = stytch::b2b::client::Client::new(
    project_id: "project-live-c60c0abe-c25a-4472-a9ed-320c6667d317",
    secret: "secret-live-80JASucyk7z_G8Z-7dVwZVGXL5NT_qGAQ2I=",
)?;
```

Create an organization

```rust
use stytch::b2b::organizations::CreateRequest;

let resp = client.organizations.create(CreateRequest{
    organization_name: String::from("Acme Co"),
    organization_slug: Some(String::from("acme-co")),
    email_allowed_domains: Some(vec![String::from("acme.co")]),
    ..Default::default()
})
.await?;
```

Log the first user into the organization

```rust
use stytch::b2b::magic_links_email::LoginOrSignupRequest;

let resp = client.magic_links.email.login_or_signup(LoginOrSignupRequest{
  organization_id: String::from("organization-id-from-create-response-..."),
  email_address: String::from("admin@acme.co"),
  ..Default::default()
})
.await?;
```

## Handling Errors

The error type for all `Result` values is `stytch::Error`. If the error is from the Stytch API,
this will be the `stytch::Error::Response` variant, which always includes an `error_type` field
you can use to identify it:

```rust
let resp = client.magic_links.authenticate(AuthenticateRequest{
    token: String::from("not-a-token!"),
    ..Default::default()
})
.await;

match resp {
    Err(stytch::Error::Response(err)) => {
        if &err.error_type == "invalid_token" {
            println!("Whoops! Try again?");
        } else {
            println!("Unexpected error type: {}", err.error_type);
        }
    }
    Err(err) => println!("Other error: {:?}", err),
    Ok(resp) => println!("Unexpected success: {:?}", resp),
}
```

Learn more about errors in the [docs](https://stytch.com/docs/api/errors).

## Cargo Features

- `reqwest-rustls-tls`: Enable reqwest's `rustls-tls` feature for the rustls implementation.
- `reqwest-native-tls`: Enable reqwest's `native-tls` feature for the native TLS implementation.
  (This is enabled by default.)

## Documentation

See example requests and responses for all the endpoints in the [Stytch API Reference](https://stytch.com/docs/api).

Follow one of the [integration guides](https://stytch.com/docs/guides) or start with one of our [example apps](https://stytch.com/docs/example-apps).

## Support

If you've found a bug, [open an issue](https://github.com/stytchauth/stytch-rust/issues/new)!

If you have questions or want help troubleshooting, join us in [Slack](https://stytch.com/docs/resources/support/overview) or email support@stytch.com.

If you've found a security vulnerability, please follow our [responsible disclosure instructions](https://stytch.com/docs/resources/security-and-trust/security#:~:text=Responsible%20disclosure%20program).

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md)

## Code of Conduct

Everyone interacting in the Stytch project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](CODE_OF_CONDUCT.md).

## Acknowledgements

Special thanks to @itsyaasir for donating the `stytch` package name to this project and starting us on our Rust journey!
