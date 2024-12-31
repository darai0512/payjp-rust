//! Customer
//! ========
//!
//! Reference: <https://pay.jp/docs/api#customers>
//!
//! This example shows how to create and list customers.

use futures_util::StreamExt;
use futures_util::TryStreamExt;
use payjp::{Client, PayjpError};
use payjp_core::customer::{CreateCustomer, ListCustomer};

pub async fn run_customer_example(client: &Client) -> Result<(), PayjpError> {
    let customer = CreateCustomer::new()
        .name("Alexander Lyon")
        .email("test@async-payjp.com")
        .description("A fake customer that is used to illustrate the examples in async-payjp.")
        .metadata([(String::from("async-payjp"), String::from("true"))])
        .send(client)
        .await?;

    println!("created a customer {}", customer.id);

    let customer = CreateCustomer::new()
        .name("Someone Else")
        .email("test@async-payjp.com")
        .description("A fake customer that is used to illustrate the examples in async-payjp.")
        .metadata([(String::from("async-payjp"), String::from("true"))])
        .send(client)
        .await?;

    println!("created a customer {}", customer.id);

    let mut stream = ListCustomer::new().paginate().stream(client);

    // get the next customer
    let _next = stream.next().await.unwrap();

    // or collect them
    let customers = stream.try_collect::<Vec<_>>().await.unwrap();

    println!("fetched {} customers: {:?}", customers.len(), customers);
    Ok(())
}