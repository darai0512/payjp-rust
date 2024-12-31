use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListStatementBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 owner: Option<ListStatementOwner>,
#[serde(skip_serializing_if = "Option::is_none")]
 source_transfer: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 term: Option<String>,
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
 type_: Option<payjp_core::StatementType>,

}
impl ListStatementBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,owner: None,source_transfer: None,tenant: None,term: None,type_: None,
    }
}

}
    /// 取引明細の発行対象で絞り込みます。以下の値が指定できます。
///
/// | owner | 絞り込み対象 |
/// | ----- | ------------ |
/// | merchant | 加盟店に向けて発行されたもの |
/// | tenant | テナントに向けて発行されたもの |
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ListStatementOwner {
Merchant,
Tenant,

}
impl ListStatementOwner {
    pub fn as_str(self) -> &'static str {
        use ListStatementOwner::*;
        match self {
Merchant => "merchant",
Tenant => "tenant",

        }
    }
}

impl std::str::FromStr for ListStatementOwner {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListStatementOwner::*;
        match s {
    "merchant" => Ok(Merchant),
"tenant" => Ok(Tenant),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ListStatementOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListStatementOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListStatementOwner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListStatementOwner {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ListStatementOwner"))
    }
}
    /// 取引明細リストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListStatement {
 inner: ListStatementBuilder,

}
impl ListStatement {
    /// Construct a new `ListStatement`.
pub fn new() -> Self {
    Self {
        inner: ListStatementBuilder::new()
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
    /// 取引明細の発行対象で絞り込みます。以下の値が指定できます。
    ///
    /// | owner | 絞り込み対象 |
    /// | ----- | ------------ |
    /// | merchant | 加盟店に向けて発行されたもの |
    /// | tenant | テナントに向けて発行されたもの |
pub fn owner(mut self, owner: impl Into<ListStatementOwner>) -> Self {
    self.inner.owner = Some(owner.into());
    self
}
    /// 取引明細の生成元オブジェクトで絞り込みます。
    ///
    /// transferオブジェクトもしくは tenant_transferオブジェクトのIDを指定します。
pub fn source_transfer(mut self, source_transfer: impl Into<String>) -> Self {
    self.inner.source_transfer = Some(source_transfer.into());
    self
}
    /// テナントのIDで絞り込みます。
pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
    self.inner.tenant = Some(tenant.into());
    self
}
    /// 集計区間のIDで絞り込みます。
pub fn term(mut self, term: impl Into<String>) -> Self {
    self.inner.term = Some(term.into());
    self
}
    /// typeの値で絞り込みます。
pub fn type_(mut self, type_: impl Into<payjp_core::StatementType>) -> Self {
    self.inner.type_ = Some(type_.into());
    self
}

}
    impl Default for ListStatement {
    fn default() -> Self {
        Self::new()
    }
}impl ListStatement {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Statement>> {
    
    payjp_client_core::ListPaginator::new_list("/statements", &self.inner)
}

}

impl PayjpRequest for ListStatement {
    type Output = payjp_types::List<payjp_core::Statement>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/statements").query(&self.inner)
}

}
    /// 取引明細を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveStatement {
 statement: payjp_core::StatementId,

}
impl RetrieveStatement {
    /// Construct a new `RetrieveStatement`.
pub fn new(statement:impl Into<payjp_core::StatementId>) -> Self {
    Self {
        statement: statement.into(),
    }
}

}
    impl RetrieveStatement {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveStatement {
    type Output = payjp_core::Statement;

    fn build(&self) -> RequestBuilder {
    let statement = &self.statement;
RequestBuilder::new(PayjpMethod::Get, format!("/statements/{statement}"))
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct StatementUrlsStatementBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 platformer: Option<bool>,

}
impl StatementUrlsStatementBuilder {
     fn new() -> Self {
    Self {
        platformer: None,
    }
}

}
        /// 取引明細およびインボイスをダウンロードできる一時URLを発行します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct StatementUrlsStatement {
 inner: StatementUrlsStatementBuilder,
 statement: payjp_core::StatementId,

}
impl StatementUrlsStatement {
    /// Construct a new `StatementUrlsStatement`.
pub fn new(statement:impl Into<payjp_core::StatementId>) -> Self {
    Self {
        statement: statement.into(),inner: StatementUrlsStatementBuilder::new()
    }
}
        /// `true`を指定するとプラットフォーム手数料に関する明細がダウンロードできるURLを発行します。.
pub fn platformer(mut self, platformer: impl Into<bool>) -> Self {
    self.inner.platformer = Some(platformer.into());
    self
}

}
    impl StatementUrlsStatement {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for StatementUrlsStatement {
    type Output = payjp_shared::StatementUrl;

    fn build(&self) -> RequestBuilder {
    let statement = &self.statement;
RequestBuilder::new(PayjpMethod::Post, format!("/statements/{statement}/statement_urls")).form(&self.inner)
}

}

