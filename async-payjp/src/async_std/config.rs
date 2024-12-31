use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use http_types::Url;
use payjp_client_core::{RequestStrategy, SharedConfigBuilder};

use crate::async_std::Client;
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
            Url::from_str(&url).map_err(|err| {
                PayjpError::ConfigError(format!("user-provided url is invalid: {err}"))
            })?
        } else {
            Url::from_str(DEFAULT_API_BASE).expect("is valid URL")
        };

        let user_agent = if let Some(app_info_str) = self.inner.app_info_str {
            format!("{DEFAULT_USER_AGENT} {app_info_str}")
        } else {
            DEFAULT_USER_AGENT.into()
        };
        println!("secret222 = {:?}", self.inner.secret);

        Ok(ClientConfig {
            user_agent,
            request_strategy: self.inner.request_strategy.unwrap_or(RequestStrategy::Once),
            secret: self.inner.secret,
            api_base,
        })
    }

    /// This method errors if any of the specified configuration is invalid.
    pub fn build(self) -> Result<Client, PayjpError> {
        Ok(Client::from_config(self.try_into_config()?))
    }
}

/// A finalized client configuration.
#[derive(Clone)]
pub struct ClientConfig {
    pub user_agent: String,
    pub request_strategy: RequestStrategy,
    pub secret: String,
    pub api_base: Url,
}

impl ClientConfig {
    pub fn to_headers_array(&self) -> [(&str, Option<&str>); 1] {
        [
            ("User-Agent", Some(&self.user_agent))
        ]
    }
}

// Don't print the secret!
impl Debug for ClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ClientConfig");
        s.field("request_strategy", &self.request_strategy);
        s.field("api_base", &self.api_base);
        s.field("user_agent", &self.user_agent);
        s.finish()
    }
}
