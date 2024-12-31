use std::fmt;
use std::fmt::{Display, Formatter};


use crate::RequestStrategy;

/// This is meant for internal use when implementing compatible clients,
/// so it may be more unstable with respect to semver.

// The disclaimer above was written to justify the semver hazard of keeping all the fields here public.
// This is not necessary, but writing accessors is tricky because configs using this
// internally want to take ownership of each field to avoid unnecessary clones.
#[derive(Clone)]
pub struct SharedConfigBuilder {
    /// The user-provided part of the user-agent we'll use.
    pub app_info_str: Option<String>,
    /// The default request strategy to use.,
    pub request_strategy: Option<RequestStrategy>,
    /// The secret key for authorizing requests.
    pub secret: String,
    /// The base URL to send requests to.
    pub api_base: Option<String>,
}

impl SharedConfigBuilder {
    /// Create a new `SharedConfigBuilder` with the given secret key.
    pub fn new(secret: impl Into<String>) -> Self {
        let secret = secret.into();

        // some basic sanity checks
        // TODO: maybe a full-blown type here rather than a warning?
        if secret.trim() != secret || !secret.starts_with("sk_") {
            tracing::warn!("suspiciously formatted secret key")
        }

        Self {
            app_info_str: None,
            request_strategy: None,
            secret,
            api_base: None,
        }
    }

    /// Sets the default `RequestStrategy` used when making requests.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.request_strategy = Some(strategy);
        self
    }

    /// Create a new client pointed at a specific URL. This is useful for testing.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.api_base = Some(url.into());
        self
    }

    /// Set the application info for the client.
    pub fn app_info(
        mut self,
        name: impl Into<String>,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        self.app_info_str = Some(AppInfo { name: name.into(), url, version }.to_string());
        self
    }
}

struct AppInfo {
    name: String,
    url: Option<String>,
    version: Option<String>,
}

impl Display for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of a User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => write!(f, "{name}/{a} ({b})"),
            (Some(a), None) => write!(f, "{name}/{a}"),
            (None, Some(b)) => write!(f, "{name} ({b})"),
            _ => f.write_str(name),
        }
    }
}

// Manual implementation so we don't print the secret!
impl fmt::Debug for SharedConfigBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("SharedConfigBuilder");
        builder.field("request_strategy", &self.request_strategy);
        builder.field("app_info_str", &self.app_info_str);
        if let Some(api_base) = &self.api_base {
            builder.field("api_base", api_base);
        }
        builder.finish()
    }
}

/// Per-request configuration overrides.
#[derive(Debug)]
#[non_exhaustive]
pub struct ConfigOverride {
    /// Use a particular `RequestStrategy`, instead of the client default.
    pub request_strategy: Option<RequestStrategy>,
}

impl ConfigOverride {
    pub(crate) fn new() -> Self {
        Self { request_strategy: None }
    }
}
