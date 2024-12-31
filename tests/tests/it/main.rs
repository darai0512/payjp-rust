// Needed for `json!` usage in tests
#![recursion_limit = "256"]

use payjp::ClientBuilder;

mod deser;
mod enums;
pub mod generated;

mod async_tests;
mod blocking;

mod deserialization_fixture;
mod pagination_utils;

pub const MOCK_LINK: &str = "http://localhost:12111";
pub const SECRET: &str = "sk_test_123";

pub fn get_base_test_config() -> ClientBuilder {
    ClientBuilder::new(SECRET).url(MOCK_LINK)
}
