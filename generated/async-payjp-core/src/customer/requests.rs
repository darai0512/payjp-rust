use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,

}
impl ListCustomerBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,
    }
}

}
            /// 生成した顧客情報のリストを取得します。リストは、直近で生成された順番に取得されます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListCustomer {
 inner: ListCustomerBuilder,

}
impl ListCustomer {
    /// Construct a new `ListCustomer`.
pub fn new() -> Self {
    Self {
        inner: ListCustomerBuilder::new()
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

}
    impl Default for ListCustomer {
    fn default() -> Self {
        Self::new()
    }
}impl ListCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Customer>> {
    
    payjp_client_core::ListPaginator::new_list("/customers", &self.inner)
}

}

impl PayjpRequest for ListCustomer {
    type Output = payjp_types::List<payjp_core::Customer>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/customers").query(&self.inner)
}

}
#[derive(Clone,Debug,serde::Serialize)]
 struct CreateCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 card: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 description: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 email: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 id: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 metadata: Option<std::collections::HashMap<String, String>>,

}
impl CreateCustomerBuilder {
     fn new() -> Self {
    Self {
        card: None,description: None,email: None,id: None,metadata: None,
    }
}

}
            /// メールアドレスやIDなどを指定して顧客を作成します。作成と同時にカード情報を登録する場合、トークンIDを指定します。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateCustomer {
 inner: CreateCustomerBuilder,

}
impl CreateCustomer {
    /// Construct a new `CreateCustomer`.
pub fn new() -> Self {
    Self {
        inner: CreateCustomerBuilder::new()
    }
}
    /// トークンID(トークンIDを指定)
pub fn card(mut self, card: impl Into<String>) -> Self {
    self.inner.card = Some(card.into());
    self
}
    /// 概要
pub fn description(mut self, description: impl Into<String>) -> Self {
    self.inner.description = Some(description.into());
    self
}
    /// メールアドレス
pub fn email(mut self, email: impl Into<String>) -> Self {
    self.inner.email = Some(email.into());
    self
}
    /// 顧客ID
pub fn id(mut self, id: impl Into<String>) -> Self {
    self.inner.id = Some(id.into());
    self
}
pub fn metadata(mut self,
                metadata: impl Into<std::collections::HashMap<String, String>>
) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}

}
    impl Default for CreateCustomer {
    fn default() -> Self {
        Self::new()
    }
}impl CreateCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateCustomer {
    type Output = payjp_core::Customer;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/customers").form(&self.inner)
}

}
    /// 生成した顧客情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveCustomer {
 customer: payjp_core::CustomerId,

}
impl RetrieveCustomer {
    /// Construct a new `RetrieveCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>) -> Self {
    Self {
        customer: customer.into(),
    }
}

}
    impl RetrieveCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveCustomer {
    type Output = payjp_core::Customer;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
RequestBuilder::new(PayjpMethod::Get, format!("/customers/{customer}"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct UpdateCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 card: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 default_card: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 description: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 email: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
metadata: Option<std::collections::HashMap<String, String>>,

}
impl UpdateCustomerBuilder {
     fn new() -> Self {
    Self {
        card: None,default_card: None,description: None,email: None,metadata: None,
    }
}

}
            /// 生成した顧客情報を更新したり、新たなカードを顧客に追加することができます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct UpdateCustomer {
 inner: UpdateCustomerBuilder,
 customer: payjp_core::CustomerId,

}
impl UpdateCustomer {
    /// Construct a new `UpdateCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>) -> Self {
    Self {
        customer: customer.into(),inner: UpdateCustomerBuilder::new()
    }
}
    /// トークンID(トークンIDを指定)
pub fn card(mut self, card: impl Into<String>) -> Self {
    self.inner.card = Some(card.into());
    self
}
    /// 保持しているカードID
pub fn default_card(mut self, default_card: impl Into<String>) -> Self {
    self.inner.default_card = Some(default_card.into());
    self
}
    /// 概要
pub fn description(mut self, description: impl Into<String>) -> Self {
    self.inner.description = Some(description.into());
    self
}
    /// メールアドレス
pub fn email(mut self, email: impl Into<String>) -> Self {
    self.inner.email = Some(email.into());
    self
}
pub fn metadata(mut self,
                metadata: impl Into<std::collections::HashMap<String, String>>
) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}

}
    impl UpdateCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for UpdateCustomer {
    type Output = payjp_core::Customer;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
RequestBuilder::new(PayjpMethod::Post, format!("/customers/{customer}")).form(&self.inner)
}

}
        /// 顧客を削除します。顧客に紐付く定期課金がある場合は、同時にそれらの定期課金も削除されます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct DeleteCustomer {
 customer: payjp_core::CustomerId,

}
impl DeleteCustomer {
    /// Construct a new `DeleteCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>) -> Self {
    Self {
        customer: customer.into(),
    }
}

}
    impl DeleteCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for DeleteCustomer {
    type Output = payjp_shared::DeleteResponse;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
RequestBuilder::new(PayjpMethod::Delete, format!("/customers/{customer}"))
}

}
#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListCardCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,

}
impl ListCardCustomerBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,
    }
}

}
            /// 顧客の保持しているカードリストを取得します。リストは、直近で生成された順番に取得されます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListCardCustomer {
 inner: ListCardCustomerBuilder,
 customer: payjp_core::CustomerId,

}
impl ListCardCustomer {
    /// Construct a new `ListCardCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>) -> Self {
    Self {
        customer: customer.into(),inner: ListCardCustomerBuilder::new()
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

}
    impl ListCardCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Card>> {
    let customer = &self.customer;

    payjp_client_core::ListPaginator::new_list(format!("/customers/{customer}/cards"), &self.inner)
}

}

impl PayjpRequest for ListCardCustomer {
    type Output = payjp_types::List<payjp_core::Card>;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
RequestBuilder::new(PayjpMethod::Get, format!("/customers/{customer}/cards")).query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateCardCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 card: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 default: Option<bool>,
#[serde(skip_serializing_if = "Option::is_none")]
metadata: Option<std::collections::HashMap<String, String>>,

}
impl CreateCardCustomerBuilder {
     fn new() -> Self {
    Self {
        card: None,default: None,metadata: None,
    }
}

}
        /// トークンIDを指定して、新たにカードを追加します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateCardCustomer {
 inner: CreateCardCustomerBuilder,
 customer: payjp_core::CustomerId,

}
impl CreateCardCustomer {
    /// Construct a new `CreateCardCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>) -> Self {
    Self {
        customer: customer.into(),inner: CreateCardCustomerBuilder::new()
    }
}
    /// トークンID
pub fn card(mut self, card: impl Into<String>) -> Self {
    self.inner.card = Some(card.into());
    self
}
    /// メイン利用のカードに設定するかどうか
pub fn default(mut self, default: impl Into<bool>) -> Self {
    self.inner.default = Some(default.into());
    self
}
pub fn metadata(mut self,
                metadata: impl Into<std::collections::HashMap<String, String>>,
) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}

}
    impl CreateCardCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateCardCustomer {
    type Output = payjp_core::Card;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
RequestBuilder::new(PayjpMethod::Post, format!("/customers/{customer}/cards")).form(&self.inner)
}

}
    /// 顧客の特定のカード情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveCardCustomer {
 customer: payjp_core::CustomerId,
 card: String,

}
impl RetrieveCardCustomer {
    /// Construct a new `RetrieveCardCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>,card:impl Into<String>) -> Self {
    Self {
        customer: customer.into(),card: card.into(),
    }
}

}
    impl RetrieveCardCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveCardCustomer {
    type Output = payjp_core::Card;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
let card = &self.card;
RequestBuilder::new(PayjpMethod::Get, format!("/customers/{customer}/cards/{card}"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct UpdateCardCustomerBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 address_city: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 address_line1: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 address_line2: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 address_state: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 address_zip: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 country: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 email: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
metadata: Option<std::collections::HashMap<String, String>>,
#[serde(skip_serializing_if = "Option::is_none")]
 name: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 phone: Option<String>,

}
impl UpdateCardCustomerBuilder {
     fn new() -> Self {
    Self {
        address_city: None,address_line1: None,address_line2: None,address_state: None,address_zip: None,country: None,email: None,metadata: None,name: None,phone: None,
    }
}

}
        /// 顧客の特定のカード情報を更新します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct UpdateCardCustomer {
 inner: UpdateCardCustomerBuilder,
 customer: payjp_core::CustomerId,
 card: String,

}
impl UpdateCardCustomer {
    /// Construct a new `UpdateCardCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>,card:impl Into<String>) -> Self {
    Self {
        customer: customer.into(),card: card.into(),inner: UpdateCardCustomerBuilder::new()
    }
}
    /// 市区町村
pub fn address_city(mut self, address_city: impl Into<String>) -> Self {
    self.inner.address_city = Some(address_city.into());
    self
}
    /// 番地など
pub fn address_line1(mut self, address_line1: impl Into<String>) -> Self {
    self.inner.address_line1 = Some(address_line1.into());
    self
}
    /// 建物名など
pub fn address_line2(mut self, address_line2: impl Into<String>) -> Self {
    self.inner.address_line2 = Some(address_line2.into());
    self
}
    /// 都道府県
pub fn address_state(mut self, address_state: impl Into<String>) -> Self {
    self.inner.address_state = Some(address_state.into());
    self
}
    /// 郵便番号
pub fn address_zip(mut self, address_zip: impl Into<String>) -> Self {
    self.inner.address_zip = Some(address_zip.into());
    self
}
    /// 2桁のISOコード(e.g. JP)
pub fn country(mut self, country: impl Into<String>) -> Self {
    self.inner.country = Some(country.into());
    self
}
    /// メールアドレス
pub fn email(mut self, email: impl Into<String>) -> Self {
    self.inner.email = Some(email.into());
    self
}
pub fn metadata(mut self,
                metadata: impl Into<std::collections::HashMap<String, String>>,
) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
    /// カード保有者名
pub fn name(mut self, name: impl Into<String>) -> Self {
    self.inner.name = Some(name.into());
    self
}
    /// E.164形式の電話番号
pub fn phone(mut self, phone: impl Into<String>) -> Self {
    self.inner.phone = Some(phone.into());
    self
}

}
    impl UpdateCardCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for UpdateCardCustomer {
    type Output = payjp_core::Card;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
let card = &self.card;
RequestBuilder::new(PayjpMethod::Post, format!("/customers/{customer}/cards/{card}")).form(&self.inner)
}

}
    /// 顧客の特定のカードを削除します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct DeleteCardCustomer {
 customer: payjp_core::CustomerId,
 card: String,

}
impl DeleteCardCustomer {
    /// Construct a new `DeleteCardCustomer`.
pub fn new(customer:impl Into<payjp_core::CustomerId>,card:impl Into<String>) -> Self {
    Self {
        customer: customer.into(),card: card.into(),
    }
}

}
    impl DeleteCardCustomer {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for DeleteCardCustomer {
    type Output = payjp_shared::DeleteResponse;

    fn build(&self) -> RequestBuilder {
    let customer = &self.customer;
let card = &self.card;
RequestBuilder::new(PayjpMethod::Delete, format!("/customers/{customer}/cards/{card}"))
}

}

