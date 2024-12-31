use payjp::blocking::Client;

use crate::get_base_test_config;

mod account;
mod charge;
mod customer;
mod plan_interval;
mod subscription;
mod token;

pub fn get_client() -> Client {
    get_base_test_config().build_sync().expect("could not build client")
}
