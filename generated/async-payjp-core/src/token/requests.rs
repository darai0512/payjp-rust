use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateTokenBuilder {
#[serde(rename = "card[address_city]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_address_city: Option<String>,
#[serde(rename = "card[address_line1]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_address_line1: Option<String>,
#[serde(rename = "card[address_line2]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_address_line2: Option<String>,
#[serde(rename = "card[address_state]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_address_state: Option<String>,
#[serde(rename = "card[address_zip]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_address_zip: Option<String>,
#[serde(rename = "card[country]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_country: Option<String>,
#[serde(rename = "card[cvc]")]
 card_cvc: String,
#[serde(rename = "card[email]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_email: Option<String>,
#[serde(rename = "card[exp_month]")]
 card_exp_month: String,
#[serde(rename = "card[exp_year]")]
 card_exp_year: String,
#[serde(rename = "card[name]")]
 card_name: String,
#[serde(rename = "card[number]")]
 card_number: String,
#[serde(rename = "card[phone]")]
#[serde(skip_serializing_if = "Option::is_none")]
 card_phone: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 tenant: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 three_d_secure: Option<bool>,

}
impl CreateTokenBuilder {
     fn new(card_cvc: impl Into<String>,card_exp_month: impl Into<String>,card_exp_year: impl Into<String>,card_name: impl Into<String>,card_number: impl Into<String>,) -> Self {
    Self {
        card_address_city: None,card_address_line1: None,card_address_line2: None,card_address_state: None,card_address_zip: None,card_country: None,card_cvc: card_cvc.into(),card_email: None,card_exp_month: card_exp_month.into(),card_exp_year: card_exp_year.into(),card_name: card_name.into(),card_number: card_number.into(),card_phone: None,tenant: None,three_d_secure: None,
    }
}

}
        /// Create new token
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateToken {
 inner: CreateTokenBuilder,

}
impl CreateToken {
    /// Construct a new `CreateToken`.
pub fn new(card_cvc:impl Into<String>,card_exp_month:impl Into<String>,card_exp_year:impl Into<String>,card_name:impl Into<String>,card_number:impl Into<String>) -> Self {
    Self {
        inner: CreateTokenBuilder::new(card_cvc.into(),card_exp_month.into(),card_exp_year.into(),card_name.into(),card_number.into(),)
    }
}
pub fn card_address_city(mut self, card_address_city: impl Into<String>) -> Self {
    self.inner.card_address_city = Some(card_address_city.into());
    self
}
pub fn card_address_line1(mut self, card_address_line1: impl Into<String>) -> Self {
    self.inner.card_address_line1 = Some(card_address_line1.into());
    self
}
pub fn card_address_line2(mut self, card_address_line2: impl Into<String>) -> Self {
    self.inner.card_address_line2 = Some(card_address_line2.into());
    self
}
pub fn card_address_state(mut self, card_address_state: impl Into<String>) -> Self {
    self.inner.card_address_state = Some(card_address_state.into());
    self
}
pub fn card_address_zip(mut self, card_address_zip: impl Into<String>) -> Self {
    self.inner.card_address_zip = Some(card_address_zip.into());
    self
}
pub fn card_country(mut self, card_country: impl Into<String>) -> Self {
    self.inner.card_country = Some(card_country.into());
    self
}
    /// メールアドレス
        /// 2024年8月以降、3Dセキュア認証の際にphoneまたはemailのデータ入力が求められます。.
pub fn card_email(mut self, card_email: impl Into<String>) -> Self {
    self.inner.card_email = Some(card_email.into());
    self
}
    /// E.164形式の電話番号 (e.g. 090-0123-4567（日本） => "+819001234567")
        /// 2024年8月以降、3Dセキュア認証の際にphoneまたはemailのデータ入力が求められます。.
pub fn card_phone(mut self, card_phone: impl Into<String>) -> Self {
    self.inner.card_phone = Some(card_phone.into());
    self
}
    /// テナントID
pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
    self.inner.tenant = Some(tenant.into());
    self
}
    /// 3Dセキュアを開始するかどうか
        /// 管理画面でトークン3Dセキュアを実施するモードが有効になっている時は無視されます。.
pub fn three_d_secure(mut self, three_d_secure: impl Into<bool>) -> Self {
    self.inner.three_d_secure = Some(three_d_secure.into());
    self
}

}
    impl CreateToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateToken {
    type Output = payjp_core::Token;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/tokens").form(&self.inner)
}

}
    /// Info for a specific token
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveToken {
 token: payjp_core::TokenId,

}
impl RetrieveToken {
    /// Construct a new `RetrieveToken`.
pub fn new(token:impl Into<payjp_core::TokenId>) -> Self {
    Self {
        token: token.into(),
    }
}

}
    impl RetrieveToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveToken {
    type Output = payjp_core::Token;

    fn build(&self) -> RequestBuilder {
    let token = &self.token;
RequestBuilder::new(PayjpMethod::Get, format!("/tokens/{token}"))
}

}

