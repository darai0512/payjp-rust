use payjp_core::token::{CreateToken};

use super::get_client;

#[test]
fn create_account_token_smoke_test() {
    let client = get_client();

    let token = CreateToken::new().send_blocking(&client).unwrap();
    assert!(!token.used);
    assert!(!token.livemode);
}
