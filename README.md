# Pay.jp OpenAPI spec & rust binding

[![crates.io](https://img.shields.io/crates/v/payjp-rust.svg)](https://crates.io/crates/payjp)
[![docs.rs](https://docs.rs/payjp-rust/badge.svg)](https://docs.rs/payjp-rust)

based on [arlyon/async-stripe](https://github.com/arlyon/async-stripe) by Alexander Lyon

## Example

This asynchronous example uses `Tokio` to create
a [Payjp Customer](https://pay.jp/docs/api#customers). Your `Cargo.toml` could
look like this:

```toml
tokio = { version = "1", features = ["full"] }
payjp = { version = "0.28", features = ["runtime-tokio-hyper"] }
payjp_core = { version = "0.28", features = ["customer"] }
```

And then the code:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_key = "sk_xxx";
    let client = payjp::Client::new(secret_key);
    let customer = payjp_core::customer::CreateCustomer {
        email: Some("test@async-payjp.com"),
        ..Default::default()
    }
        .send(&client)
        .await?;

    println!("created a customer at https://pay.jp/d/customers/{}", customer.id);
    Ok(())
}
```

A full list of examples can be found in the [examples](/examples).

## Relevant Crates

### Client

The main entry point is the `payjp-rust` crate which provides a client for making Payjp
API requests.
`payjp-rust` is compatible with the [`async-std`](https://github.com/async-rs/async-std)
and [`tokio`](https://github.com/tokio-rs/tokio) runtimes and the `native-tls`
and `rustls` backends. When adding the dependency, you must select a runtime feature.

#### Installation

```toml
[dependencies]
payjp = { version = "0.31", features = ["runtime-tokio-hyper"] }
```

#### Feature Flags

supports the following features combining runtime and TLS choices:

- `runtime-tokio-hyper`
- `runtime-tokio-hyper-rustls`
- `runtime-tokio-hyper-rustls-webpki`
- `runtime-blocking`
- `runtime-blocking-rustls`
- `runtime-blocking-rustls-webpki`
- `runtime-async-std-surf`