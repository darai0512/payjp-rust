use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListTermBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since_start_at: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until_start_at: Option<i64>,

}
impl ListTermBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,since_start_at: None,until_start_at: None,
    }
}

}
        /// 集計区間（Termオブジェクト）のリストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListTerm {
 inner: ListTermBuilder,

}
impl ListTerm {
    /// Construct a new `ListTerm`.
pub fn new() -> Self {
    Self {
        inner: ListTermBuilder::new()
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
    /// start_atが指定したタイムスタンプ以降のオブジェクトを取得
pub fn since_start_at(mut self, since_start_at: impl Into<i64>) -> Self {
    self.inner.since_start_at = Some(since_start_at.into());
    self
}
    /// start_atが指定したタイムスタンプ以前のオブジェクトを取得
pub fn until_start_at(mut self, until_start_at: impl Into<i64>) -> Self {
    self.inner.until_start_at = Some(until_start_at.into());
    self
}

}
    impl Default for ListTerm {
    fn default() -> Self {
        Self::new()
    }
}impl ListTerm {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Term>> {
    
    payjp_client_core::ListPaginator::new_list("/terms", &self.inner)
}

}

impl PayjpRequest for ListTerm {
    type Output = payjp_types::List<payjp_core::Term>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/terms").query(&self.inner)
}

}
    /// 指定されたIDの集計区間（Termオブジェクト）を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveTerm {
 term: payjp_core::TermId,

}
impl RetrieveTerm {
    /// Construct a new `RetrieveTerm`.
pub fn new(term:impl Into<payjp_core::TermId>) -> Self {
    Self {
        term: term.into(),
    }
}

}
    impl RetrieveTerm {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveTerm {
    type Output = payjp_core::Term;

    fn build(&self) -> RequestBuilder {
    let term = &self.term;
RequestBuilder::new(PayjpMethod::Get, format!("/terms/{term}"))
}

}

