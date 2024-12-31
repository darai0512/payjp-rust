use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListThreeDSecureRequestBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 resource_id: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant_id: Option<String>,

}
impl ListThreeDSecureRequestBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,resource_id: None,tenant_id: None,
    }
}

}
        /// 3Dセキュアリクエスト情報のリストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListThreeDSecureRequest {
 inner: ListThreeDSecureRequestBuilder,

}
impl ListThreeDSecureRequest {
    /// Construct a new `ListThreeDSecureRequest`.
pub fn new() -> Self {
    Self {
        inner: ListThreeDSecureRequestBuilder::new()
    }
}
    /// 取得するデータ数の最大値(1~100まで)。指定がない場合は 10 となる。
pub fn limit(mut self, limit: impl Into<i64>) -> Self {
    self.inner.limit = Some(limit.into());
    self
}
    /// 基準点からのデータ取得を行う開始位置。指定がない場合は 0 となる。
pub fn offset(mut self, offset: impl Into<i64>) -> Self {
    self.inner.offset = Some(offset.into());
    self
}
    /// ここに指定したタイムスタンプ以降に作成されたデータを取得
pub fn since(mut self, since: impl Into<i64>) -> Self {
    self.inner.since = Some(since.into());
    self
}
    /// ここに指定したタイムスタンプ以前に作成されたデータを取得
pub fn until(mut self, until: impl Into<i64>) -> Self {
    self.inner.until = Some(until.into());
    self
}
    /// 3Dセキュア処理対象リソースのID
pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
    self.inner.resource_id = Some(resource_id.into());
    self
}
    /// テナントID
pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
    self.inner.tenant_id = Some(tenant_id.into());
    self
}

}
    impl Default for ListThreeDSecureRequest {
    fn default() -> Self {
        Self::new()
    }
}impl ListThreeDSecureRequest {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::ThreeDSecureRequest>> {
    
    payjp_client_core::ListPaginator::new_list("/three_d_secure_requests", &self.inner)
}

}

impl PayjpRequest for ListThreeDSecureRequest {
    type Output = payjp_types::List<payjp_core::ThreeDSecureRequest>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/three_d_secure_requests").query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateThreeDSecureRequestBuilder {
 resource_id: String,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant_id: Option<String>,

}
impl CreateThreeDSecureRequestBuilder {
     fn new(resource_id: impl Into<String>,) -> Self {
    Self {
        resource_id: resource_id.into(),tenant_id: None,
    }
}

}
        /// 顧客カードIDを指定して3Dセキュアリクエストを作成します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateThreeDSecureRequest {
 inner: CreateThreeDSecureRequestBuilder,

}
impl CreateThreeDSecureRequest {
    /// Construct a new `CreateThreeDSecureRequest`.
pub fn new(resource_id:impl Into<String>) -> Self {
    Self {
        inner: CreateThreeDSecureRequestBuilder::new(resource_id.into(),)
    }
}
    /// テナントID。PAY.JP Platform のみ設定可能
pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
    self.inner.tenant_id = Some(tenant_id.into());
    self
}

}
    impl CreateThreeDSecureRequest {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateThreeDSecureRequest {
    type Output = payjp_core::ThreeDSecureRequest;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/three_d_secure_requests").form(&self.inner)
}

}
    /// 3Dセキュアリクエスト情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveThreeDSecureRequest {
 three_d_secure_request: payjp_core::ThreeDSecureRequestId,

}
impl RetrieveThreeDSecureRequest {
    /// Construct a new `RetrieveThreeDSecureRequest`.
pub fn new(three_d_secure_request:impl Into<payjp_core::ThreeDSecureRequestId>) -> Self {
    Self {
        three_d_secure_request: three_d_secure_request.into(),
    }
}

}
    impl RetrieveThreeDSecureRequest {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveThreeDSecureRequest {
    type Output = payjp_core::ThreeDSecureRequest;

    fn build(&self) -> RequestBuilder {
    let three_d_secure_request = &self.three_d_secure_request;
RequestBuilder::new(PayjpMethod::Get, format!("/three_d_secure_requests/{three_d_secure_request}"))
}

}

