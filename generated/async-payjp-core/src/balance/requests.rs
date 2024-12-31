use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListBalanceBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since_due_date: Option<payjp_types::Timestamp>,
#[serde(skip_serializing_if = "Option::is_none")]
 until_due_date: Option<payjp_types::Timestamp>,
#[serde(skip_serializing_if = "Option::is_none")]
 state: Option<payjp_core::BalanceState>,
#[serde(skip_serializing_if = "Option::is_none")]
 closed: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
 owner: Option<ListBalanceOwner>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant: Option<String>,

}
impl ListBalanceBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,since_due_date: None,until_due_date: None,state: None,closed: None,owner: None,tenant: None,
    }
}

}
    /// Balanceの所有者で絞り込みます。以下の値が指定できます。`merchant`または`tenant`.
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ListBalanceOwner {
Merchant,
Tenant,

}
impl ListBalanceOwner {
    pub fn as_str(self) -> &'static str {
        use ListBalanceOwner::*;
        match self {
Merchant => "merchant",
Tenant => "tenant",

        }
    }
}

impl std::str::FromStr for ListBalanceOwner {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListBalanceOwner::*;
        match s {
    "merchant" => Ok(Merchant),
"tenant" => Ok(Tenant),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ListBalanceOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListBalanceOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListBalanceOwner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListBalanceOwner {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ListBalanceOwner"))
    }
}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListBalance {
 inner: ListBalanceBuilder,

}
impl ListBalance {
    /// Construct a new `ListBalance`.
pub fn new() -> Self {
    Self {
        inner: ListBalanceBuilder::new()
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
    /// 入金予定日/振込期限日が指定したタイムスタンプ以降のデータのみ取得
pub fn since_due_date(mut self, since_due_date: impl Into<payjp_types::Timestamp>) -> Self {
    self.inner.since_due_date = Some(since_due_date.into());
    self
}
    /// 入金予定日/振込期限日が指定したタイムスタンプ以前のデータのみ取得
pub fn until_due_date(mut self, until_due_date: impl Into<payjp_types::Timestamp>) -> Self {
    self.inner.until_due_date = Some(until_due_date.into());
    self
}
    /// stateが指定した値であるオブジェクトに限定
pub fn state(mut self, state: impl Into<payjp_core::BalanceState>) -> Self {
    self.inner.state = Some(state.into());
    self
}
    /// closedが指定した値であるオブジェクトに限定
pub fn closed(mut self, closed: impl Into<bool>) -> Self {
    self.inner.closed = Some(closed.into());
    self
}
        /// Balanceの所有者で絞り込みます。以下の値が指定できます。`merchant`または`tenant`.
pub fn owner(mut self, owner: impl Into<ListBalanceOwner>) -> Self {
    self.inner.owner = Some(owner.into());
    self
}
    /// 指定したテナントが所有者であるオブジェクトに限定
pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
    self.inner.tenant = Some(tenant.into());
    self
}

}
    impl Default for ListBalance {
    fn default() -> Self {
        Self::new()
    }
}impl ListBalance {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for ListBalance {
    type Output = ListBalanceReturned;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/balances").query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ListBalanceReturned {
pub data: Option<Vec<payjp_core::Balance>>,

}
#[doc(hidden)]
pub struct ListBalanceReturnedBuilder {
    data: Option<Option<Vec<payjp_core::Balance>>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for ListBalanceReturned {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<ListBalanceReturned>,
    builder: ListBalanceReturnedBuilder,
}

impl Visitor for Place<ListBalanceReturned> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ListBalanceReturnedBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ListBalanceReturnedBuilder {
    type Out = ListBalanceReturned;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "data" => Deserialize::begin(&mut self.data),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            data: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(data),
) = (self.data.take(),
) else {
            return None;
        };
        Some(Self::Out { data })
    }
}

impl<'a> Map for Builder<'a> {
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        self.builder.key(k)
    }

    fn finish(&mut self) -> Result<()> {
        *self.out = self.builder.take_out();
        Ok(())
    }
}

impl ObjectDeser for ListBalanceReturned {
    type Builder = ListBalanceReturnedBuilder;
}

impl FromValueOpt for ListBalanceReturned {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ListBalanceReturnedBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "data" => b.data = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
    /// 特定の残高オブジェクトを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveBalance {
 balance: payjp_core::BalanceId,

}
impl RetrieveBalance {
    /// Construct a new `RetrieveBalance`.
pub fn new(balance:impl Into<payjp_core::BalanceId>) -> Self {
    Self {
        balance: balance.into(),
    }
}

}
    impl RetrieveBalance {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveBalance {
    type Output = payjp_core::Balance;

    fn build(&self) -> RequestBuilder {
    let balance = &self.balance;
RequestBuilder::new(PayjpMethod::Get, format!("/balances/{balance}"))
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct StatementUrlsBalanceBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 platformer: Option<bool>,

}
impl StatementUrlsBalanceBuilder {
     fn new() -> Self {
    Self {
        platformer: None,
    }
}

}
            /// Balanceに含まれるStatementすべての取引明細およびインボイスを一括ダウンロードできる一時URLを発行します。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct StatementUrlsBalance {
 inner: StatementUrlsBalanceBuilder,
 balance: payjp_core::BalanceId,

}
impl StatementUrlsBalance {
    /// Construct a new `StatementUrlsBalance`.
pub fn new(balance:impl Into<payjp_core::BalanceId>) -> Self {
    Self {
        balance: balance.into(),inner: StatementUrlsBalanceBuilder::new()
    }
}
        /// `true`を指定するとプラットフォーム手数料に関する明細がダウンロードできるURLを発行します。.
pub fn platformer(mut self, platformer: impl Into<bool>) -> Self {
    self.inner.platformer = Some(platformer.into());
    self
}

}
    impl StatementUrlsBalance {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for StatementUrlsBalance {
    type Output = payjp_shared::StatementUrl;

    fn build(&self) -> RequestBuilder {
    let balance = &self.balance;
RequestBuilder::new(PayjpMethod::Post, format!("/balances/{balance}/statement_urls")).form(&self.inner)
}

}

