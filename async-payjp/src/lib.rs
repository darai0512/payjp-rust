// Copyright 2019 Wyyerd Group, LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/async-payjp/")]
#![recursion_limit = "128"]

//! This crate provides Rust bindings to the Payjp HTTP API.
//!
//! ## Getting Started
//!
//! To get started, we need to create a [Client]:
//!
//! ```
//! # #[cfg(feature = "__hyper")]
//! let client = payjp::Client::new("sk_YOUR_SECRET");
//! ```
//!
//! Then we can begin making requests as we'd like.  Most requests accept
//! many optional parameters, so we usually get the `::new(...)` with any required
//! params and then set the ones we want from there.
//!
//! Most requests for creating or updating an object use the same Rust struct,
//! so you may frequently need to refer to the [official API docs](https://pay.jp/docs/api)
//! to determine which fields are required for either request.
//!
//! > **Note:** We have an extensive collection of examples which are interspersed in
//! > the documentation. Any time an API is used in an example it is highlighted in the
//! > docs for that item. You can also find all the raw examples in the `examples` directory.
//! > Please have a look at those for inspiration or ideas on how to get started.
//!
//! ## Idempotency / Request Strategies
//!
//! This library provides a few basic request strategies for making requests to the Payjp API.
//! This is currently implemented as an enum with the following variants:
//!
//! - [`RequestStrategy::Once`]: This is the default strategy. It will make a request to the API and,
//!                              whether the request fails or not, will simply return the response.
//! - [`RequestStrategy::Retry`]: Make a request to the API and, if the request fails, retry it up to n
//!                               times with a timeout. The idempotency key is generated automatically and is
//!                               stable across retries.
//! - [`RequestStrategy::ExponentialBackoff`]: Make a request to the API and, if the request fails, retry
//!                                            it up to n times with exponential backoff. The idempotency key is
//!                                            generated automatically and is stable across retries.
//!
//! > Want to implement your own? If it is a common strategy, please consider opening a PR to add it to the library.
//! > Otherwise, we are open to turning this into an open trait so that you can implement your own strategy.

#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![deny(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

mod error;

#[cfg(feature = "async-std-surf")]
pub mod async_std;

#[cfg(feature = "__hyper")]
mod hyper;
pub use error::PayjpError;
#[cfg(feature = "__hyper")]
pub use hyper::*;
pub use payjp_client_core::{
    CustomizedPayjpRequest, ListPaginator, PaginationExt, RequestStrategy,
};
pub use payjp_shared::error::*;
