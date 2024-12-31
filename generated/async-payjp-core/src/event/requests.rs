use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListEventBuilder {
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
 object: Option<String>,
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
 type_: Option<String>,

}
impl ListEventBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,resource_id: None,object: None,type_: None,
    }
}

}
        /// イベントリストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListEvent {
 inner: ListEventBuilder,

}
impl ListEvent {
    /// Construct a new `ListEvent`.
pub fn new() -> Self {
    Self {
        inner: ListEventBuilder::new()
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
    /// 取得するeventに紐づくAPIリソースのID (e.g. customer.id)
pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
    self.inner.resource_id = Some(resource_id.into());
    self
}
        /// 取得するeventに紐づくAPIリソースのobject。値はリソース名(e.g.
    /// customer, charge).
pub fn object(mut self, object: impl Into<String>) -> Self {
    self.inner.object = Some(object.into());
    self
}
    /// 取得するeventのtype
pub fn type_(mut self, type_: impl Into<String>) -> Self {
    self.inner.type_ = Some(type_.into());
    self
}

}
    impl Default for ListEvent {
    fn default() -> Self {
        Self::new()
    }
}impl ListEvent {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_shared::Event>> {
    
    payjp_client_core::ListPaginator::new_list("/events", &self.inner)
}

}

impl PayjpRequest for ListEvent {
    type Output = payjp_types::List<payjp_shared::Event>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/events").query(&self.inner)
}

}
    /// 特定のイベント情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveEvent {
 event: payjp_shared::EventId,

}
impl RetrieveEvent {
    /// Construct a new `RetrieveEvent`.
pub fn new(event:impl Into<payjp_shared::EventId>) -> Self {
    Self {
        event: event.into(),
    }
}

}
    impl RetrieveEvent {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveEvent {
    type Output = payjp_shared::Event;

    fn build(&self) -> RequestBuilder {
    let event = &self.event;
RequestBuilder::new(PayjpMethod::Get, format!("/events/{event}"))
}

}

