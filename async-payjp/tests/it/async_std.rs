use http_types::convert::{Deserialize, Serialize};
use httpmock::prelude::*;
use payjp::async_std::{Client, ClientBuilder};
use payjp::PayjpError;
use payjp_client_core::{
    CustomizablePayjpRequest, RequestBuilder, RequestStrategy, PayjpMethod,
};
use payjp_shared::ApiErrorsType::InvalidRequestError;

fn client_builder() -> ClientBuilder {
    ClientBuilder::new("secret")
}

fn get_client_for(mock_server: &MockServer) -> Client {
    client_builder().url(mock_server.base_url()).build().expect("could not build client")
}

fn server_errors_req() -> CustomizablePayjpRequest<()> {
    RequestBuilder::new(PayjpMethod::Get, "/server-errors").customize()
}

#[derive(Debug, Serialize, Deserialize)]
struct TestData {
    // Allowing dead code since used for deserialization
    #[allow(dead_code)]
    id: String,
}

const TEST_DATA_ID: &str = "test-id";

impl TestData {
    pub fn new() -> Self {
        Self { id: TEST_DATA_ID.into() }
    }
}

#[async_std::test]
async fn retry() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let req = server_errors_req().request_strategy(RequestStrategy::Retry(5));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

#[async_std::test]
async fn user_error() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/missing");
        then.status(404).body("{
                \"error\": {
                  \"message\": \"Unrecognized request URL (GET: /v1/missing).\",
                  \"type\": \"invalid_request_error\"
                }
              }
              ");
    });

    let req = RequestBuilder::new(PayjpMethod::Get, "/missing")
        .customize::<()>()
        .request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    mock.assert_hits_async(1).await;

    match res {
        Err(PayjpError::Payjp(x, status)) => {
            assert_eq!(status, 404);
            assert_eq!(x.type_, InvalidRequestError);
            assert!(x.message.unwrap().contains("Unrecognized"));
        }
        _ => panic!("Expected error, got {:?}", res),
    }
}

#[async_std::test]
async fn retry_header() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500).header("X-TEST", "true");
    });

    let req = server_errors_req().request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(1).await;
    assert!(res.is_err());
}

#[async_std::test]
async fn retry_body() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/server-errors")
            .header("content-type", "application/x-www-form-urlencoded")
            .x_www_form_urlencoded_tuple("id", TEST_DATA_ID);
        then.status(500);
    });

    let req = RequestBuilder::new(PayjpMethod::Post, "/server-errors")
        .form(&TestData::new())
        .customize::<()>()
        .request_strategy(RequestStrategy::Retry(5));
    let res = req.send(&client).await;

    hello_mock.assert_hits_async(5).await;
    assert!(res.is_err());
}
