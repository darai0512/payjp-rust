use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListSubscriptionBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 plan: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 customer: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 status: Option<ListSubscriptionStatus>,

}
impl ListSubscriptionBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,plan: None,customer: None,status: None,
    }
}

}
    /// 定期課金ステータス。`active`, `trial`, `canceled` または `paused` のみ指定可能。.
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ListSubscriptionStatus {
Active,
Trial,
Canceled,
Paused,

}
impl ListSubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        use ListSubscriptionStatus::*;
        match self {
Active => "active",
Trial => "trial",
Canceled => "canceled",
Paused => "paused",

        }
    }
}

impl std::str::FromStr for ListSubscriptionStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListSubscriptionStatus::*;
        match s {
    "active" => Ok(Active),
"trial" => Ok(Trial),
"canceled" => Ok(Canceled),
"paused" => Ok(Paused),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListSubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListSubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListSubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ListSubscriptionStatus"))
    }
}
    /// 生成した定期課金のリストを取得します。
    ///
    /// リストは、直近で生成された順番に取得されます。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListSubscription {
 inner: ListSubscriptionBuilder,

}
impl ListSubscription {
    /// Construct a new `ListSubscription`.
pub fn new() -> Self {
    Self {
        inner: ListSubscriptionBuilder::new()
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
    /// 絞り込みたいプランID
pub fn plan(mut self, plan: impl Into<String>) -> Self {
    self.inner.plan = Some(plan.into());
    self
}
    /// 絞り込みたい顧客ID
pub fn customer(mut self, customer: impl Into<String>) -> Self {
    self.inner.customer = Some(customer.into());
    self
}
        /// 定期課金ステータス。`active`, `trial`, `canceled` または `paused` のみ指定可能。.
pub fn status(mut self, status: impl Into<ListSubscriptionStatus>) -> Self {
    self.inner.status = Some(status.into());
    self
}

}
    impl Default for ListSubscription {
    fn default() -> Self {
        Self::new()
    }
}impl ListSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Subscription>> {
    
    payjp_client_core::ListPaginator::new_list("/subscriptions", &self.inner)
}

}

impl PayjpRequest for ListSubscription {
    type Output = payjp_types::List<payjp_core::Subscription>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/subscriptions").query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateSubscriptionBuilder {
 customer: String,
#[serde(skip_serializing_if = "Option::is_none")]
 metadata: Option<payjp_shared::Metadata>,
 plan: String,
#[serde(skip_serializing_if = "Option::is_none")]
 prorate: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
 trial_end: Option<CreateSubscriptionTrialEnd>,

}
impl CreateSubscriptionBuilder {
     fn new(customer: impl Into<String>,plan: impl Into<String>,) -> Self {
    Self {
        customer: customer.into(),metadata: None,plan: plan.into(),prorate: None,trial_end: None,
    }
}

}
    /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionTrialEnd {
#[serde(untagged)]
I64(i64),
#[serde(untagged)]
String(String),

}
    /// 顧客IDとプランIDを指定して、定期課金を開始します。
    ///
        /// 前払い式のため、定期課金作成時に最初の課金が実行されます。但し以下の場合には作成時の課金はされません。.
    ///
    /// - トライアル状態(`status=trial`)で作成された場合
        /// - プランの課金日(`billing_day`)が指定され、定期課金の日割り設定(`prorate`)が設定されていない場合.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateSubscription {
 inner: CreateSubscriptionBuilder,

}
impl CreateSubscription {
    /// Construct a new `CreateSubscription`.
pub fn new(customer:impl Into<String>,plan:impl Into<String>) -> Self {
    Self {
        inner: CreateSubscriptionBuilder::new(customer.into(),plan.into(),)
    }
}
pub fn metadata(mut self, metadata: impl Into<payjp_shared::Metadata>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
    /// `true`の場合、日割り課金を設定します
pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
    self.inner.prorate = Some(prorate.into());
    self
}
        /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
pub fn trial_end(mut self, trial_end: impl Into<CreateSubscriptionTrialEnd>) -> Self {
    self.inner.trial_end = Some(trial_end.into());
    self
}

}
    impl CreateSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/subscriptions").form(&self.inner)
}

}
    /// 指定されたIDの定期課金情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveSubscription {
 subscription: String,

}
impl RetrieveSubscription {
    /// Construct a new `RetrieveSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),
    }
}

}
    impl RetrieveSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Get, format!("/subscriptions/{subscription}"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct UpdateSubscriptionBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 metadata: Option<payjp_shared::Metadata>,
#[serde(skip_serializing_if = "Option::is_none")]
 next_cycle_plan: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 plan: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 prorate: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
 trial_end: Option<UpdateSubscriptionTrialEnd>,

}
impl UpdateSubscriptionBuilder {
     fn new() -> Self {
    Self {
        metadata: None,next_cycle_plan: None,plan: None,prorate: None,trial_end: None,
    }
}

}
    /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionTrialEnd {
#[serde(untagged)]
I64(i64),
#[serde(untagged)]
String(String),

}
        /// トライアル期間を新たに設定したり、プランの変更を行うことができます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct UpdateSubscription {
 inner: UpdateSubscriptionBuilder,
 subscription: String,

}
impl UpdateSubscription {
    /// Construct a new `UpdateSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),inner: UpdateSubscriptionBuilder::new()
    }
}
pub fn metadata(mut self, metadata: impl Into<payjp_shared::Metadata>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
        /// 次回サイクル更新時に指定のプランへと自動的に切り替えを行い課金を試みます。.
pub fn next_cycle_plan(mut self, next_cycle_plan: impl Into<String>) -> Self {
    self.inner.next_cycle_plan = Some(next_cycle_plan.into());
    self
}
    /// 新しいプランのID
pub fn plan(mut self, plan: impl Into<String>) -> Self {
    self.inner.plan = Some(plan.into());
    self
}
    /// `true`の場合、日割り課金を設定します
pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
    self.inner.prorate = Some(prorate.into());
    self
}
        /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
pub fn trial_end(mut self, trial_end: impl Into<UpdateSubscriptionTrialEnd>) -> Self {
    self.inner.trial_end = Some(trial_end.into());
    self
}

}
    impl UpdateSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for UpdateSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Post, format!("/subscriptions/{subscription}")).form(&self.inner)
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct DeleteSubscriptionBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 prorate: Option<bool>,

}
impl DeleteSubscriptionBuilder {
     fn new() -> Self {
    Self {
        prorate: None,
    }
}

}
        /// 定期課金をすぐに削除します。
        /// 次回以降の課金は行われず、一度削除した定期課金は再び戻すことができません。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct DeleteSubscription {
 inner: DeleteSubscriptionBuilder,
 subscription: String,

}
impl DeleteSubscription {
    /// Construct a new `DeleteSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),inner: DeleteSubscriptionBuilder::new()
    }
}
        /// `true`の場合、削除時から現在の周期の終了日までの日割り分を算出し、返金処理を行います。.
pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
    self.inner.prorate = Some(prorate.into());
    self
}

}
    impl DeleteSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for DeleteSubscription {
    type Output = payjp_shared::DeleteResponse;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Delete, format!("/subscriptions/{subscription}")).form(&self.inner)
}

}
        /// 引き落としの失敗やカードが不正である、また定期課金を停止したい場合はこのリクエストで定期購入を停止させます。.
    ///
        /// 定期課金を停止させると、再開されるまで引き落とし処理は一切行われません。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct PauseSubscription {
 subscription: String,

}
impl PauseSubscription {
    /// Construct a new `PauseSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),
    }
}

}
    impl PauseSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for PauseSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Post, format!("/subscriptions/{subscription}/pause"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ResumeSubscriptionBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 prorate: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
 trial_end: Option<ResumeSubscriptionTrialEnd>,

}
impl ResumeSubscriptionBuilder {
     fn new() -> Self {
    Self {
        prorate: None,trial_end: None,
    }
}

}
    /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeSubscriptionTrialEnd {
#[serde(untagged)]
I64(i64),
#[serde(untagged)]
String(String),

}
        /// 停止もしくはキャンセル状態(`status=canceled or paused`)の定期課金を再開させます。.
    ///
    /// トライアル期間中であればトライアル状態(`status=trial`)で再開します。
    ///
        /// 再開時の `current_period_end` が過去の日時の場合、トライアル期間内でなければ支払いが行われ、その時点が周期の開始として設定されます。.
        /// 支払いの失敗により停止していた場合などは、 `current_period_end` は支払い失敗時の値になるため、必ず過去の日時がセットされます。.
    ///
    /// 再開時の支払いに失敗すると、定期課金は再開されません。
        /// この場合は、有効なカードを顧客のデフォルトカードにセットしてから、再度定期課金の再開を行ってください。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ResumeSubscription {
 inner: ResumeSubscriptionBuilder,
 subscription: String,

}
impl ResumeSubscription {
    /// Construct a new `ResumeSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),inner: ResumeSubscriptionBuilder::new()
    }
}
    /// `true`の場合、日割り課金を設定します
pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
    self.inner.prorate = Some(prorate.into());
    self
}
        /// リクエスト時より未来のタイムスタンプ(5年後まで) or 文字列 `now` が指定可能です。.
pub fn trial_end(mut self, trial_end: impl Into<ResumeSubscriptionTrialEnd>) -> Self {
    self.inner.trial_end = Some(trial_end.into());
    self
}

}
    impl ResumeSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for ResumeSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Post, format!("/subscriptions/{subscription}/resume")).form(&self.inner)
}

}
        /// 定期課金をキャンセルし、現在の周期の終了日をもって定期課金を終了させます。.
    ///
        /// 終了日以前であれば、定期課金の再開リクエスト(/resume)を行うことで、キャンセルを取り消すことができます。.
    /// 終了日をむかえた定期課金は自動的に削除されますのでご注意ください。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CancelSubscription {
 subscription: String,

}
impl CancelSubscription {
    /// Construct a new `CancelSubscription`.
pub fn new(subscription:impl Into<String>) -> Self {
    Self {
        subscription: subscription.into(),
    }
}

}
    impl CancelSubscription {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CancelSubscription {
    type Output = payjp_core::Subscription;

    fn build(&self) -> RequestBuilder {
    let subscription = &self.subscription;
RequestBuilder::new(PayjpMethod::Post, format!("/subscriptions/{subscription}/cancel"))
}

}

