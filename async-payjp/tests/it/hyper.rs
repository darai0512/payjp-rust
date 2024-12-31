use httpmock::Method::{GET, POST};
use httpmock::MockServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use payjp::{Client, ClientBuilder, RequestStrategy, PayjpError};
use payjp_client_core::{CustomizablePayjpRequest, RequestBuilder, PayjpMethod};
use payjp_shared::ErrorErrorType::InvalidRequestError;

fn client_builder() -> ClientBuilder {
    ClientBuilder::new("secret")
}

fn get_client_for(mock_server: &MockServer) -> Client {
    client_builder().url(mock_server.base_url()).build().expect("could not build client")
}

fn server_errors_req() -> CustomizablePayjpRequest<()> {
    RequestBuilder::new(PayjpMethod::Get, "/server-errors").customize()
}

fn test_req() -> CustomizablePayjpRequest<TestData> {
    RequestBuilder::new(PayjpMethod::Get, "/test").customize()
}

#[derive(miniserde::Deserialize, Debug, Serialize, Deserialize)]
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

#[tokio::test]
async fn retry() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = client_builder()
        .url(server.base_url())
        .request_strategy(RequestStrategy::Retry(5))
        .build()
        .unwrap();

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let res = server_errors_req().send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());

    // And it should also work for retry strategy set per request
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500);
    });

    let res = server_errors_req().request_strategy(RequestStrategy::Retry(5)).send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

#[tokio::test]
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

    let res = RequestBuilder::new(PayjpMethod::Get, "/missing")
        .customize::<TestData>()
        .request_strategy(RequestStrategy::Retry(3))
        .send(&client)
        .await;

    // We don't retry a 404
    mock.assert_hits_async(1).await;

    match res {
        Err(PayjpError::Payjp(x, status)) => {
            assert_eq!(status, 404);
            assert_eq!(x.type_, InvalidRequestError);
            assert!(x.message.contains("Unrecognized"));
        }
        _ => panic!("Expected error"),
    }
}

#[tokio::test]
async fn nice_serde_error() {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, miniserde::Deserialize)]
    struct DataType {
        // Allowing dead code since used for deserialization
        #[allow(dead_code)]
        id: String,
        #[allow(dead_code)]
        name: String,
    }

    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/odd_data");
        then.status(200).body(
            "{
                \"id\": \"test\",
                \"name\": 10
              }
              ",
        );
    });

    let req = RequestBuilder::new(PayjpMethod::Get, "/odd_data")
        .customize::<DataType>()
        .request_strategy(RequestStrategy::Retry(3));
    let res = req.send(&client).await;

    mock.assert_hits_async(1).await;

    match res {
        Err(PayjpError::JSONDeserialize(_)) => {}
        _ => panic!("Expected error {:?}", res),
    }
}

#[tokio::test]
async fn retry_header() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/v1/server-errors");
        then.status(500).header("Should-Retry", "false");
    });

    let res = server_errors_req().send(&client).await;

    hello_mock.assert_hits_async(1).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn retry_with_body() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    // Create a mock on the server.
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/server-errors")
            .header("content-type", "application/x-www-form-urlencoded")
            .x_www_form_urlencoded_tuple("id", TEST_DATA_ID);
        then.status(500);
    });

    let req = RequestBuilder::new(PayjpMethod::Post, "/server-errors").form(&TestData::new());
    let res =
        req.customize::<TestData>().request_strategy(RequestStrategy::Retry(5)).send(&client).await;

    mock.assert_hits_async(5).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn user_error_transfers() {
    // Start a lightweight mock server.
    let server = MockServer::start_async().await;
    let client = get_client_for(&server);

    let message = "test";
    let mock = server.mock(|when, then| {
        when.method(GET).path("/v1/transfers");
        then.status(400).json_body(json!({
          "error": {
            "code": "insufficient_capabilities_for_transfer",
            "message": message,
            "type": "invalid_request_error"
          }
        }));
    });

    let req = RequestBuilder::new(PayjpMethod::Get, "/transfers").customize::<()>();
    let res = req.send(&client).await.unwrap_err();
    mock.assert_hits_async(1).await;

    match res {
        PayjpError::Payjp(err, status_code) => {
            assert_eq!(status_code, 400);
            assert_eq!(err.type_, InvalidRequestError);
            assert_eq!(err.message, message);
        }
        _ => panic!("Expected error, got {:?}", res),
    }
}

async fn assert_headers_sent(
    builder: ClientBuilder,
    req: CustomizablePayjpRequest<TestData>,
    expected: Vec<(impl Into<String>, impl Into<String>)>,
) {
    let server = MockServer::start_async().await;
    let client = builder.url(server.base_url()).build().unwrap();

    let mock = server.mock(|mut when, then| {
        when = when.method(GET).path("/v1/test");
        for (name, value) in expected {
            when = when.header(name, value);
        }

        then.status(200).json_body_obj(&TestData::new());
    });

    let _ = req.send(&client).await;
    mock.assert_hits_async(1).await;
}

async fn assert_user_agent_sent(builder: ClientBuilder, expected: String) {
    assert_headers_sent(builder, test_req(), vec![("user-agent", expected)]).await
}

/// Check user agent format.
#[tokio::test]
async fn user_agent() {
    assert_user_agent_sent(
        client_builder(),
        format!("Payjp/v1 RustBindings/{}", env!("CARGO_PKG_VERSION")),
    )
    .await;
    assert_user_agent_sent(
        client_builder().app_info("sick-new-startup", None, None),
        format!("Payjp/v1 RustBindings/{} sick-new-startup", env!("CARGO_PKG_VERSION")),
    )
    .await;
    assert_user_agent_sent(
        client_builder().app_info(
            "sick-new-startup",
            Some("0.1.0".to_string()),
            Some("https://sick-startup.io".to_string()),
        ),
        format!(
            "Payjp/v1 RustBindings/{} sick-new-startup/0.1.0 (https://sick-startup.io)",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .await;
}

async fn assert_header_ids(
    builder: ClientBuilder,
    req: CustomizablePayjpRequest<TestData>,
) {
    let mut headers: Vec<(String, String)> = vec![];
    assert_headers_sent(builder, req, headers).await;
}

#[tokio::test]
async fn tokio_test() {
    let both = client_builder();
    assert_header_ids(both.clone(), test_req()).await;

    // We should also be able to override the account id on a per-request basis
    assert_header_ids(
        both,
        test_req(),
    )
    .await;
}
