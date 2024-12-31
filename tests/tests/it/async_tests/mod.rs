use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

use bytes::Bytes;
use futures_util::TryFutureExt;
use payjp::CustomizedPayjpRequest;

use crate::{SECRET};

mod pagination;

/// For testing async clients generically. This could be avoided if we could use
/// `&dyn PayjpClient`, but currently traits with async functions cannot be object safe.
#[derive(Clone)]
pub enum PayjpClient {
    Hyper(payjp::Client),
    AsyncStd(Box<payjp::async_std::Client>),
}

#[derive(Debug)]
// Code is used for debugging errors!
#[allow(dead_code)]
pub struct SimplePayjpClientError(String);

impl client_core::PayjpClientErr for SimplePayjpClientError {
    fn deserialize_err(msg: impl Display) -> Self {
        SimplePayjpClientError(msg.to_string())
    }
}

impl client_core::PayjpClient for PayjpClient {
    type Err = SimplePayjpClientError;

    // See AFIT stabilization PR (https://github.com/rust-lang/rust/pull/115822)
    #[allow(refining_impl_trait)]
    fn execute(
        &self,
        req: CustomizedPayjpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Bytes, SimplePayjpClientError>> + '_>> {
        match self {
            PayjpClient::Hyper(c) => {
                Box::pin(c.execute(req).map_err(|err| SimplePayjpClientError(err.to_string())))
            }
            PayjpClient::AsyncStd(c) => {
                Box::pin(c.execute(req).map_err(|err| SimplePayjpClientError(err.to_string())))
            }
        }
    }
}

impl PayjpClient {
    fn hyper() -> Self {
        Self::Hyper(payjp::ClientBuilder::new(SECRET).url(MOCK_LINK).build().unwrap())
    }

    fn async_std() -> Self {
        Self::AsyncStd(Box::new(
            payjp::async_std::ClientBuilder::new(SECRET).url(MOCK_LINK).build().unwrap(),
        ))
    }
}

/// Run the test closure on all async clients
pub fn test_with_all_clients<TestFn, Fut>(test: TestFn)
where
    TestFn: Fn(PayjpClient) -> Fut,
    Fut: Future<Output = ()>,
{
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        test(PayjpClient::hyper()).await;
        test(PayjpClient::async_std()).await;
    });
}
