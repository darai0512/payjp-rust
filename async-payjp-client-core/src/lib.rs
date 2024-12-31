//! This crate provides shared utilities for implementing clients

#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]
mod config;
mod pagination;
mod request_strategy;
mod request;

pub use config::{ConfigOverride, SharedConfigBuilder};
pub use pagination::*;
pub use request_strategy::*;
pub use request::*;
