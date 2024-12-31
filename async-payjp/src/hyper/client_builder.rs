use hyper::http::{HeaderValue, Uri};
use payjp_client_core::{RequestStrategy, SharedConfigBuilder};
use base64::{engine::general_purpose, Engine as _};

use crate::hyper::client::Client;
use crate::PayjpError;

static DEFAULT_USER_AGENT: &str = concat!("Payjp/v1 RustBindings/", env!("CARGO_PKG_VERSION"));
const DEFAULT_API_BASE: &str = "https://api.pay.jp/";

/// Configuration for a client.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    inner: SharedConfigBuilder,
}

impl ClientBuilder {
    /// Create a new `ClientBuilder` with the given secret key.
    pub fn new(secret: impl Into<String>) -> Self {
        Self { inner: SharedConfigBuilder::new(secret) }
    }

    /// Set the default `RequestStrategy` used when making requests.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.inner = self.inner.request_strategy(strategy);
        self
    }

    /// Create a new client pointed at a specific URL. This is useful for testing.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.inner = self.inner.url(url);
        self
    }

    /// Set the application info for the client.
    pub fn app_info(
        mut self,
        name: impl Into<String>,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        self.inner = self.inner.app_info(name, version, url);
        self
    }

    fn try_into_config(self) -> Result<ClientConfig, PayjpError> {
        let api_base = if let Some(url) = self.inner.api_base {
            Uri::try_from(url).map_err(|err| {
                PayjpError::ConfigError(format!("user-provided url is invalid: {err}"))
            })?
        } else {
            Uri::from_static(DEFAULT_API_BASE)
        };

        let user_agent_header = if let Some(app_info_str) = self.inner.app_info_str {
            HeaderValue::try_from(format!("{DEFAULT_USER_AGENT} {app_info_str}"))
                .map_err(|_| cons_header_err("app_info"))?
        } else {
            HeaderValue::from_static(DEFAULT_USER_AGENT)
        };

        let encoded = general_purpose::STANDARD.encode(format!("{}:", self.inner.secret));
        let mut secret = HeaderValue::try_from(format!("Basic {}", encoded))
            .map_err(|_| cons_header_err("secret"))?;
        secret.set_sensitive(true);
        Ok(ClientConfig {
            user_agent: user_agent_header,
            request_strategy: self.inner.request_strategy.unwrap_or(RequestStrategy::Once),
            secret,
            api_base,
        })
    }

    /// This method errors if any of the specified configuration is invalid.
    pub fn build(self) -> Result<Client, PayjpError> {
        Ok(Client::from_config(self.try_into_config()?))
    }

    /// This method errors if any of the specified configuration is invalid.
    /// This method panics if called from within an async runtime.
    #[cfg(feature = "blocking")]
    pub fn build_sync(self) -> Result<crate::hyper::blocking::Client, PayjpError> {
        Ok(crate::hyper::blocking::Client::from_async(self.build()?))
    }
}

fn cons_header_err(config_name: &'static str) -> PayjpError {
    PayjpError::ClientError(format!("`{config_name}` can only include visible ASCII characters"))
}

/// A validated client configuration. All configuration types are carefully chosen to be
/// cheaply clonable so that the client can be cheaply cloned.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub user_agent: HeaderValue,
    pub request_strategy: RequestStrategy,
    // NB: This `HeaderValue` is marked as sensitive, so it won't be debug printed.
    pub secret: HeaderValue,
    pub api_base: Uri,
}
