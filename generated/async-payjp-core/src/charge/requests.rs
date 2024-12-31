use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListChargeBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 customer: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 subscription: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 term: Option<String>,

}
impl ListChargeBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,customer: None,subscription: None,tenant: None,term: None,
    }
}

}
        /// 生成した支払い情報のリストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListCharge {
 inner: ListChargeBuilder,

}
impl ListCharge {
    /// Construct a new `ListCharge`.
pub fn new() -> Self {
    Self {
        inner: ListChargeBuilder::new()
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
    /// 絞り込みたい顧客ID
pub fn customer(mut self, customer: impl Into<String>) -> Self {
    self.inner.customer = Some(customer.into());
    self
}
    /// 絞り込みたい定期課金ID
pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
    self.inner.subscription = Some(subscription.into());
    self
}
    /// [PAY.JP Platform](#platform-api)のみ指定可能
    ///
    /// 絞り込みたいテナントID
pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
    self.inner.tenant = Some(tenant.into());
    self
}
        /// [入金管理オブジェクトの刷新に伴い、2024/06/01以降に提供されます。](https://pay.jp/docs/migrate-transfer).
    ///
    /// 絞り込みたいTermのID
pub fn term(mut self, term: impl Into<String>) -> Self {
    self.inner.term = Some(term.into());
    self
}

}
    impl Default for ListCharge {
    fn default() -> Self {
        Self::new()
    }
}impl ListCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Charge>> {
    
    payjp_client_core::ListPaginator::new_list("/charges", &self.inner)
}

}

impl PayjpRequest for ListCharge {
    type Output = payjp_types::List<payjp_core::Charge>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/charges").query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateChargeBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 amount: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 capture: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
 card: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 currency: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 customer: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 description: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 expiry_days: Option<u32>,
#[serde(skip_serializing_if = "Option::is_none")]
#[serde(with = "payjp_types::with_serde_json_opt")]
 metadata: Option<miniserde::json::Value>,
#[serde(skip_serializing_if = "Option::is_none")]
 platform_fee: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 product: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 three_d_secure: Option<bool>,

}
impl CreateChargeBuilder {
     fn new() -> Self {
    Self {
        amount: None,capture: None,card: None,currency: None,customer: None,description: None,expiry_days: None,metadata: None,platform_fee: None,product: None,tenant: None,three_d_secure: None,
    }
}

}
            /// トークンIDまたはカードを保有している顧客IDを指定して支払いを作成します。.
        /// テスト用のキーでは、本番用の決済ネットワークへは接続されず、実際の請求が行われることもありません。.
        /// 本番用のキーでは、決済ネットワークで処理が行われ、実際の請求が行われます。.
    ///
        /// 支払いを確定せずに、カードの認証と支払い額のみ確保する場合は、 `capture` に `false` を指定してください。.
        /// このとき `expiry_days` を指定することで、認証の期間を定めることができます。 `expiry_days` はデフォルトで7日となっており、1日~60日の間で設定が可能です。なお60日に設定した場合、認証期限は59日後の11:59PM(日本時間)までになります。また確保されました与信枠は、`expiry_days` で設定されました期間を過ぎると解放されるようなっております。.
    ///
    /// `three_d_secure` にtrueを指定することで、3Dセキュアを開始できます。
        /// 指定した場合、支払いオブジェクトは作成されますが実際の決済処理は保留された状態になります。.
        /// 保留中の支払いは、引数指定の内容に関わらず`captured`は`false`、`captured_at`は`null`、`expired_at`は`null`と表示されます。.
        /// なお、支払い作成から30分を経過すると、3Dセキュアフローはタイムアウトし、処理が進められなくなります。.
        /// 3Dセキュアの進め方は、 [支払いで3Dセキュアを実施する](https://pay.jp/docs/charge-tds) を参照してください。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateCharge {
 inner: CreateChargeBuilder,

}
impl CreateCharge {
    /// Construct a new `CreateCharge`.
pub fn new() -> Self {
    Self {
        inner: CreateChargeBuilder::new()
    }
}
    /// 50~9,999,999の整数
pub fn amount(mut self, amount: impl Into<i64>) -> Self {
    self.inner.amount = Some(amount.into());
    self
}
    /// 支払い処理を確定するかどうか
pub fn capture(mut self, capture: impl Into<bool>) -> Self {
    self.inner.capture = Some(capture.into());
    self
}
    /// トークンIDまたはカードID
pub fn card(mut self, card: impl Into<String>) -> Self {
    self.inner.card = Some(card.into());
    self
}
    /// 3文字のISOコード(現状 "jpy" のみサポート)
pub fn currency(mut self, currency: impl Into<String>) -> Self {
    self.inner.currency = Some(currency.into());
    self
}
    /// 顧客ID
pub fn customer(mut self, customer: impl Into<String>) -> Self {
    self.inner.customer = Some(customer.into());
    self
}
    /// 概要
pub fn description(mut self, description: impl Into<String>) -> Self {
    self.inner.description = Some(description.into());
    self
}
    /// 認証状態が失効するまでの日数
pub fn expiry_days(mut self, expiry_days: impl Into<u32>) -> Self {
    self.inner.expiry_days = Some(expiry_days.into());
    self
}
    /// キーバリューの任意データ
pub fn metadata(mut self, metadata: impl Into<miniserde::json::Value>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
    /// [PAY.JP Platform](#platform-api) のみ設定可能
    ///
    /// プラットフォーマーに振り分けられる入金金額。
pub fn platform_fee(mut self, platform_fee: impl Into<i64>) -> Self {
    self.inner.platform_fee = Some(platform_fee.into());
    self
}
    /// プロダクトID
pub fn product(mut self, product: impl Into<String>) -> Self {
    self.inner.product = Some(product.into());
    self
}
    /// [PAY.JP Platform](#platform-api) のみ設定可能
    ///
    /// テナントID
pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
    self.inner.tenant = Some(tenant.into());
    self
}
    /// 3Dセキュアを行うならtrue
pub fn three_d_secure(mut self, three_d_secure: impl Into<bool>) -> Self {
    self.inner.three_d_secure = Some(three_d_secure.into());
    self
}

}
    impl Default for CreateCharge {
    fn default() -> Self {
        Self::new()
    }
}impl CreateCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/charges").form(&self.inner)
}

}
    /// 生成された支払い情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveCharge {
 charge: payjp_core::ChargeId,

}
impl RetrieveCharge {
    /// Construct a new `RetrieveCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>) -> Self {
    Self {
        charge: charge.into(),
    }
}

}
    impl RetrieveCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Get, format!("/charges/{charge}"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct UpdateChargeBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 description: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
#[serde(with = "payjp_types::with_serde_json_opt")]
 metadata: Option<miniserde::json::Value>,

}
impl UpdateChargeBuilder {
     fn new() -> Self {
    Self {
        description: None,metadata: None,
    }
}

}
        /// 支払い情報を更新します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct UpdateCharge {
 inner: UpdateChargeBuilder,
 charge: payjp_core::ChargeId,

}
impl UpdateCharge {
    /// Construct a new `UpdateCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>) -> Self {
    Self {
        charge: charge.into(),inner: UpdateChargeBuilder::new()
    }
}
    /// 概要
pub fn description(mut self, description: impl Into<String>) -> Self {
    self.inner.description = Some(description.into());
    self
}
    /// キーバリューの任意データ
pub fn metadata(mut self, metadata: impl Into<miniserde::json::Value>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}

}
    impl UpdateCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for UpdateCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Post, format!("/charges/{charge}")).form(&self.inner)
}

}
    /// 3Dセキュア認証が終了した支払いに対し、決済を行います。
        /// [支払いを作成](#支払いを作成) と同様の決済処理が実行され、実際の請求が行われる状態になります。カードの状態によっては支払いに失敗し、402エラーとなる点も同様です。.
        /// 保留中の支払いで固定値となっていた`capture`、`captured_at`、`expired_at`は、支払い作成時に指定した通りに反映されます。`captured_at`、`expired_at`の時刻は本APIにリクエストした時刻を基準として設定されます。.
    ///
        /// `three_d_secure_status`が`verified`、`attempted`でない支払いに対して本APIをリクエストした場合、エラーとなります。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct TdsFinishCharge {
 charge: payjp_core::ChargeId,

}
impl TdsFinishCharge {
    /// Construct a new `TdsFinishCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>) -> Self {
    Self {
        charge: charge.into(),
    }
}

}
    impl TdsFinishCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for TdsFinishCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Post, format!("/charges/{charge}/tds_finish"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct RefundChargeBuilder {
 amount: i64,
#[serde(skip_serializing_if = "Option::is_none")]
 refund_reason: Option<String>,

}
impl RefundChargeBuilder {
     fn new(amount: impl Into<i64>,) -> Self {
    Self {
        amount: amount.into(),refund_reason: None,
    }
}

}
            /// 支払い済みとなった処理を返金します。全額返金、及び `amount` を指定することで金額の部分返金を行うことができます。また確定していない支払いも取り消すことができますが `amount` を指定して部分返金をすることはできません。.
    ///
    /// なお返金可能な期限につきましては売上作成より`180日以内`となります。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RefundCharge {
 inner: RefundChargeBuilder,
 charge: payjp_core::ChargeId,

}
impl RefundCharge {
    /// Construct a new `RefundCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>,amount:impl Into<i64>) -> Self {
    Self {
        charge: charge.into(),inner: RefundChargeBuilder::new(amount.into(),)
    }
}
    /// 返金理由 (255文字以内)
pub fn refund_reason(mut self, refund_reason: impl Into<String>) -> Self {
    self.inner.refund_reason = Some(refund_reason.into());
    self
}

}
    impl RefundCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RefundCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Post, format!("/charges/{charge}/refund")).form(&self.inner)
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct ReauthChargeBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 expiry_days: Option<u32>,

}
impl ReauthChargeBuilder {
     fn new() -> Self {
    Self {
        expiry_days: None,
    }
}

}
        /// **各種SDKは順次対応予定です**
    ///
        /// 認証状態となった処理待ちの支払いを再認証します。 `captured="false"` の支払いが対象です。.
        /// `expiry_days` を指定することで、新たな認証の期間を定めることができます。 `expiry_days` はデフォルトで7日となっており、1日~60日の間で設定が可能です。なお60日に設定した場合、認証期限は59日後の11:59PM(日本時間)までになります。.
    ///
        /// **再認証が必要な場合は認証状態の charge を[返金し](#返金する)、新たに[支払いを作成](#支払いを作成) することを推奨いたします。**.
    ///
        /// このAPIは認証済みの与信をキャンセルせず別の与信を作るため、同じ金額で認証済みでも失敗したり、デビットカードなどでは一度目の認証(capture=falseの支払い)と含めて二重に金額が引き落とされることがあります。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ReauthCharge {
 inner: ReauthChargeBuilder,
 charge: payjp_core::ChargeId,

}
impl ReauthCharge {
    /// Construct a new `ReauthCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>) -> Self {
    Self {
        charge: charge.into(),inner: ReauthChargeBuilder::new()
    }
}
    /// 認証状態が失効するまでの日数
pub fn expiry_days(mut self, expiry_days: impl Into<u32>) -> Self {
    self.inner.expiry_days = Some(expiry_days.into());
    self
}

}
    impl ReauthCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for ReauthCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Post, format!("/charges/{charge}/reauth")).form(&self.inner)
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct CaptureChargeBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 amount: Option<i64>,

}
impl CaptureChargeBuilder {
     fn new() -> Self {
    Self {
        amount: None,
    }
}

}
            /// 認証状態となった処理待ちの支払い処理を確定させます。具体的には `captured="false"` となった支払いが該当します。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CaptureCharge {
 inner: CaptureChargeBuilder,
 charge: payjp_core::ChargeId,

}
impl CaptureCharge {
    /// Construct a new `CaptureCharge`.
pub fn new(charge:impl Into<payjp_core::ChargeId>) -> Self {
    Self {
        charge: charge.into(),inner: CaptureChargeBuilder::new()
    }
}
    /// 50~9,999,999の整数
    ///
        /// これをセットすることで、支払い生成時の金額と異なる金額の支払い処理を行うことができます。.
        /// ただし支払い生成時の金額よりも少額である必要があるためご注意ください。.
    ///
        /// セットした場合、レスポンスの `amount_refunded` に認証時の `amount` との差額が入ります。.
        /// 例えば、認証時に `amount=500` で作成し、 `amount=400` で支払い確定を行った場合、 `amount_refunded=100` となり、確定金額が400円に変更された状態で支払いが確定されます。.
pub fn amount(mut self, amount: impl Into<i64>) -> Self {
    self.inner.amount = Some(amount.into());
    self
}

}
    impl CaptureCharge {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CaptureCharge {
    type Output = payjp_core::Charge;

    fn build(&self) -> RequestBuilder {
    let charge = &self.charge;
RequestBuilder::new(PayjpMethod::Post, format!("/charges/{charge}/capture")).form(&self.inner)
}

}

