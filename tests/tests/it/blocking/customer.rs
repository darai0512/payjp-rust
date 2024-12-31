use payjp_core::{
    customer::{CreateCustomer, DeleteCustomer, RetrieveCustomer},
    CustomerId,
};

use super::{get_base_test_config, get_client};

fn customer_create_and_delete(client: &payjp::blocking::Client) {
    let customer = CreateCustomer::new().send_blocking(client).unwrap();
    let result = DeleteCustomer::new(&customer.id).send_blocking(client).unwrap();
    assert_eq!(result.id, customer.id);
}

#[test]
fn customer_create_and_delete_without_account() {
    let client = get_client();
    customer_create_and_delete(&client);
}

#[test]
#[ignore]
fn customer_create_and_delete_with_account() {
    let client = get_base_test_config()
        .build_sync()
        .unwrap();
    customer_create_and_delete(&client);
}
