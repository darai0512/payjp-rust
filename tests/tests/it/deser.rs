use miniserde::json::from_str;
use serde_json::{json, Value};
use payjp_core::Account;
use payjp_core::customer::RetrieveCustomerReturned;
use payjp_core::{
    Charge, Customer, ChargeStatus
};
use payjp_types::{Currency, Timestamp};

fn mock_customer_with_card() -> Value {
    json!({
      "id": "cus_1234",
      "object": "customer",
      "balance": 0,
      "created": 1542631579,
      "currency": null,
      "default_source": "card_ABCD",
      "delinquent": false,
      "description": "Customer for green.tea@example.com",
      "discount": null,
      "email": null,
      "invoice_prefix": "99999AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "card_ABCD",
            "object": "card",
            "address_city": null,
            "address_country": null,
            "address_line1": null,
            "address_line1_check": null,
            "address_line2": null,
            "address_state": null,
            "address_zip": null,
            "address_zip_check": null,
            "brand": "American Express",
            "country": "US",
            "customer": "cus_1234",
            "cvc_check": null,
            "dynamic_last4": null,
            "exp_month": 11,
            "exp_year": 2019,
            "fingerprint": "ffff9999ffff9999",
            "funding": "credit",
            "last4": "4242",
            "metadata": {},
            "name": null,
            "tokenization_method": null
          }
        ],
        "has_more": false,
        "count": 1,
        "url": "/v1/customers/cus_1234/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "count": 0,
        "url": "/v1/customers/cus_1234/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    })
}

#[test]
fn deserialize_customer_with_card() {
    let example = mock_customer_with_card().to_string();
    let result: Customer = from_str(&example).expect("deserialization failed");
    assert_eq!(result.balance, Some(0));
    assert!(!result.livemode);
}

#[test]
fn deserialize_customer_with_source() {
    let example = json!({
      "id": "cus_5678",
      "object": "customer",
      "account_balance": 0,
      "created": 1538150891,
      "currency": null,
      "default_source": "src_EFGH",
      "delinquent": false,
      "description": null,
      "discount": null,
      "email": null,
      "invoice_prefix": "00AA00AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "src_EFGH",
            "object": "source",
            "amount": null,
            "card": {
              "exp_month": 9,
              "exp_year": 2019,
              "brand": "Visa",
              "country": "US",
              "fingerprint": "AAAA1111aaaa2222",
              "funding": "credit",
              "last4": "4242",
              "three_d_secure": "optional",
              "name": null,
              "address_line1_check": null,
              "address_zip_check": null,
              "cvc_check": null,
              "tokenization_method": null,
              "dynamic_last4": null
            },
            "client_secret": "src_client_secret_SUPERSECRETTECH",
            "created": 1538150891,
            "currency": null,
            "customer": "cus_5678",
            "flow": "none",
            "livemode": false,
            "metadata": {},
            "owner": {
              "address": null,
              "email": null,
              "name": null,
              "phone": null,
              "verified_address": null,
              "verified_email": null,
              "verified_name": null,
              "verified_phone": null
            },
            "statement_descriptor": "My Company",
            "status": "chargeable",
            "type": "card",
            "usage": "reusable"
          }
        ],
        "has_more": false,
        "count": 1,
        "url": "/v1/customers/cus_5678/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "count": 0,
        "url": "/v1/customers/cus_5678/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    })
    .to_string();

    let result: Customer = from_str(&example).unwrap();
    assert_eq!(result.description, None);
}

#[test]
fn deserialize_checkout_event() {
    use payjp_core::Event;

    let example = json!({
      "created": 1326853478,
      "livemode": false,
      "id": "evt_00000000000000",
      "type": "checkout.session.completed",
      "object": "event",
      "request": null,
      "pending_webhooks": 1,
      "api_version": "2019-05-16",
      "data": {
        "object": {
          "id": "cs_00000000000000",
          "object": "checkout.session",
          "billing_address_collection": null,
          "cancel_url": "https://example.com/cancel",
          "client_reference_id": null,
          "customer": null,
          "customer_email": null,
          "display_items": [
            {
              "amount": 1500,
              "currency": "usd",
              "custom": {
                "description": "Comfortable cotton t-shirt",
                "images": null,
                "name": "T-shirt"
              },
              "quantity": 2,
              "type": "custom"
            }
          ],
          "livemode": false,
          "locale": null,
          "mode": "payment",
          "payment_intent": "pi_00000000000000",
          "payment_method_types": [
            "card"
          ],
          "payment_status": "paid",
          "submit_type": null,
          "subscription": null,
          "success_url": "https://example.com/success"
        }
      }
    })
    .to_string();
    let result: Event = from_str(&example).unwrap();
    assert_eq!(result.pending_webhooks, 1);
}

#[test]
fn deserialize_charge_with_no_refunds() {
    let example = json!({
      "amount": 0,
      "billing_details": {},
      "amount_captured": 0,
      "amount_refunded": 0,
      "captured": true,
      "currency": "cad",
      "created": 1703349829,
      "disputed": false,
      "object": "charge",
      "id": "ch_123",
      "livemode": false,
      "metadata": {},
      "paid": false,
      "status": "pending",
      "refunded": false,
    })
    .to_string();
    let charge: Charge = from_str(&example).unwrap();
    assert_eq!(charge.id.as_str(), "ch_123");
    assert_eq!(charge.currency, Currency::CAD);
    assert_eq!(charge.status, ChargeStatus::Pending);
    assert_eq!(charge.created, 1703349829);
}

const FILE_CREATED: Timestamp = 1704511150;
fn mock_file() -> Value {
    json!({
      "created": FILE_CREATED,
      "id": "file_123",
      "size": 0,
      "object": "file",
      "purpose": "account_requirement"
    })
}

#[test]
fn deserialize_expandable_polymorphic() {
    let base_cust = json!({
        "currency": "usd",
        "created": 1704511150,
        "id": "cus_123",
        "livemode": false,
        "object": "customer",
        "tax_exempt": "exempt",
    });

    let mut cust = base_cust.clone();
    cust.as_object_mut().unwrap().insert("default_source".into(), Value::String("ba_123".into()));

    let result: Customer = from_str(&cust.to_string()).unwrap();
    assert_eq!(result.created, 1704511150);
    assert_eq!(result.currency, Some(Currency::USD));
    assert_eq!(result.id.as_str(), "cus_123");
    assert_eq!(result.default_source.as_ref().unwrap().id().as_str(), "ba_123");
}