use std::fmt::Display;
use std::marker::PhantomData;

use bytes::Bytes;
use miniserde::json::from_str;
use serde::Serialize;

use crate::request_strategy::RequestStrategy;
use crate::ConfigOverride;

/// REST API methods used by Payjp.
#[derive(Debug, Copy, Clone)]
pub enum PayjpMethod {
    /// GET
    Get,
    /// POST
    Post,
    /// DELETE
    Delete,
}

/// This trait allows clients implementing `PayjpClient` to define their own error type,
/// while ensuring they can receive deserialization errors.
pub trait PayjpClientErr {
    /// Raised when we cannot deserialize the bytes received from a request into the
    /// specified type.
    fn deserialize_err(msg: impl Display) -> Self;
}

/// An abstraction for defining HTTP clients capable of making API requests compatible
/// with request information generated in the request crates.
pub trait PayjpClient {
    /// The error returned, either if the request failed due to an issue with client
    /// communicating with Payjp, or a client error returned by the API.
    type Err: PayjpClientErr;

    /// Make the API call.
    fn execute(
        &self,
        req: CustomizedPayjpRequest,
    ) -> impl std::future::Future<Output = Result<Bytes, Self::Err>>;
}

/// An abstraction for defining HTTP clients capable of making blocking API requests compatible
/// with request information generated in the request crates.
pub trait BlockingClient {
    /// The error returned.
    type Err: PayjpClientErr;

    /// Make a blocking API call.
    fn execute(&self, req: CustomizedPayjpRequest) -> Result<Bytes, Self::Err>;
}

/// Define how to convert structs into the data required to make a specific API call.
pub trait PayjpRequest {
    /// The data returned from the eventual API call.
    type Output;

    /// Convert the struct into library-agnostic data that can be used by compatible
    /// clients to make API calls.
    fn build(&self) -> RequestBuilder;

    /// Convert to a builder allowing per-request customization.
    fn customize(&self) -> CustomizablePayjpRequest<Self::Output> {
        CustomizablePayjpRequest::new(self.build())
    }
}

/// A `CustomizablePayjpRequest` allows for configuring per-request behavior that overrides
/// default configuration values.
#[derive(Debug)]
pub struct CustomizablePayjpRequest<T> {
    inner: CustomizedPayjpRequest,
    _output: PhantomData<T>,
}

/// The request specification used by a compatible client to make a request.
#[derive(Debug)]
pub struct CustomizedPayjpRequest {
    request: RequestBuilder,
    config_override: ConfigOverride,
}

impl CustomizedPayjpRequest {
    /// Split the request specification into the request itself and any configuration override.
    pub fn into_pieces(self) -> (RequestBuilder, ConfigOverride) {
        (self.request, self.config_override)
    }
}

impl<T> CustomizablePayjpRequest<T> {
    fn new(req_builder: RequestBuilder) -> Self {
        Self {
            _output: PhantomData,
            inner: CustomizedPayjpRequest {
                request: req_builder,
                config_override: ConfigOverride::new(),
            },
        }
    }

    /// Set a strategy to use for this request, overriding the default set
    /// during configuration.
    pub fn request_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.inner.config_override.request_strategy = Some(strategy);
        self
    }
}

impl<T: miniserde::Deserialize> CustomizablePayjpRequest<T> {
    /// Sends the request and returns the response.
    pub async fn send<C: PayjpClient>(self, client: &C) -> Result<T, C::Err> {
        let bytes = client.execute(self.inner).await?;
        deserialize_bytes(bytes)
    }

    /// Sends the request, blocking the main thread until the response is returned.
    pub fn send_blocking<C: BlockingClient>(self, client: &C) -> Result<T, C::Err> {
        let bytes = client.execute(self.inner)?;
        deserialize_bytes(bytes)
    }
}

fn deserialize_bytes<T: miniserde::Deserialize, Err: PayjpClientErr>(
    bytes: Bytes,
) -> Result<T, Err> {
    let str = std::str::from_utf8(bytes.as_ref())
        .map_err(|_| Err::deserialize_err("Response was not valid UTF-8"))?;
    from_str(str).map_err(|_| Err::deserialize_err("error deserializing request data"))
}

/// A builder for specifying the possible pieces of a API request.
#[derive(Debug)]
pub struct RequestBuilder {
    /// The current query string to use, if provided.
    pub query: Option<String>,
    /// The current form-encoded body to send, if provided.
    pub body: Option<String>,
    /// The API endpoint to send the request to.
    pub path: String,
    /// The method type.
    pub method: PayjpMethod,
}

impl RequestBuilder {
    /// Construct a new `RequestBuilder`.
    pub fn new(method: PayjpMethod, path: impl Into<String>) -> Self {
        Self { path: path.into(), method, query: None, body: None }
    }

    /// Set a query by serializing the params.
    #[allow(clippy::missing_panics_doc)]
    pub fn query<P: Serialize>(mut self, params: &P) -> Self {
        self.query = Some(serde_qs::to_string(params).expect("valid serialization"));
        self
    }

    /// Construct a serialized, form-encoded body.
    #[allow(clippy::missing_panics_doc)]
    pub fn form<F: Serialize>(mut self, form: &F) -> Self {
        self.body = Some(serde_qs::to_string(form).expect("valid serialization"));
        self
    }

    /// Convert this request into a `CustomizablePayjpRequest` to allow per-request
    /// customization.
    pub fn customize<T>(self) -> CustomizablePayjpRequest<T> {
        CustomizablePayjpRequest::new(self)
    }
}
