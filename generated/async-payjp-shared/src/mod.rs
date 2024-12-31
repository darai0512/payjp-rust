#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]

//!

extern crate self as payjp_shared;

miniserde::make_place!(Place);
#[doc(hidden)]
pub mod delete_response;#[doc(inline)]
pub use delete_response::*;#[doc(hidden)]
pub mod error;#[doc(inline)]
pub use error::*;#[doc(hidden)]
pub mod event;#[doc(inline)]
pub use event::*;#[doc(hidden)]
pub mod metadata;#[doc(inline)]
pub use metadata::*;#[doc(hidden)]
pub mod statement_url;#[doc(inline)]
pub use statement_url::*;