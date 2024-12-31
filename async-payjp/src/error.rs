use std::fmt::Display;

use payjp_client_core::PayjpClientErr;
use payjp_shared::ErrorError;
use thiserror::Error;

/// An error encountered from server.
#[derive(Debug, Error)]
pub enum PayjpError {
    /// Server returned a client error.
    #[error("error reported by payjp: {0:#?}, status code: {1}")]
    Payjp(Box<ErrorError>, u16),
    /// An error occurred when parsing the response.
    #[error("error deserializing a request: {0}")]
    JSONDeserialize(String),
    /// An error occurred communicating with server.
    #[error("error communicating with server: {0}")]
    ClientError(String),
    /// The client configuration was invalid.
    #[error("configuration error: {0}")]
    ConfigError(String),
    /// A blocking request timed out
    #[error("timeout communicating with server")]
    Timeout,
}

impl PayjpClientErr for PayjpError {
    fn deserialize_err(msg: impl Display) -> Self {
        Self::JSONDeserialize(msg.to_string())
    }
}

#[cfg(feature = "__hyper")]
impl From<hyper::Error> for PayjpError {
    fn from(err: hyper::Error) -> PayjpError {
        PayjpError::ClientError(err.to_string())
    }
}

#[cfg(feature = "__hyper")]
impl From<hyper::http::Error> for PayjpError {
    fn from(err: hyper::http::Error) -> PayjpError {
        PayjpError::ClientError(err.to_string())
    }
}

#[cfg(feature = "async-std-surf")]
impl From<http_types::Error> for PayjpError {
    fn from(err: http_types::Error) -> PayjpError {
        PayjpError::ClientError(err.to_string())
    }
}
