use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

#[derive(Copy,Clone,Debug,)]#[derive(serde::Serialize)]
 struct ListTenantBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 limit: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 offset: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 since: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 until: Option<i64>,

}
impl ListTenantBuilder {
     fn new() -> Self {
    Self {
        limit: None,offset: None,since: None,until: None,
    }
}

}
        /// テナントのリストを取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ListTenant {
 inner: ListTenantBuilder,

}
impl ListTenant {
    /// Construct a new `ListTenant`.
pub fn new() -> Self {
    Self {
        inner: ListTenantBuilder::new()
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
    impl Default for ListTenant {
    fn default() -> Self {
        Self::new()
    }
}impl ListTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(&self) -> payjp_client_core::ListPaginator<payjp_types::List<payjp_core::Tenant>> {
    
    payjp_client_core::ListPaginator::new_list("/tenants", &self.inner)
}

}

impl PayjpRequest for ListTenant {
    type Output = payjp_types::List<payjp_core::Tenant>;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/tenants").query(&self.inner)
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct CreateTenantBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_holder_name: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_number: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_type: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_branch_code: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_code: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 id: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 metadata: Option<payjp_shared::Metadata>,
#[serde(skip_serializing_if = "Option::is_none")]
 minimum_transfer_amount: Option<i64>,
 name: String,
#[serde(skip_serializing_if = "Option::is_none")]
 payjp_fee_included: Option<bool>,
 platform_fee_rate: f64,

}
impl CreateTenantBuilder {
     fn new(name: impl Into<String>,platform_fee_rate: impl Into<f64>,) -> Self {
    Self {
        bank_account_holder_name: None,bank_account_number: None,bank_account_type: None,bank_branch_code: None,bank_code: None,id: None,metadata: None,minimum_transfer_amount: None,name: name.into(),payjp_fee_included: None,platform_fee_rate: platform_fee_rate.into(),
    }
}

}
        /// 名前やIDなどを指定してテナントを作成します。
    ///
    /// 作成したテナントはあとから更新・削除することができます。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct CreateTenant {
 inner: CreateTenantBuilder,

}
impl CreateTenant {
    /// Construct a new `CreateTenant`.
pub fn new(name:impl Into<String>,platform_fee_rate:impl Into<f64>) -> Self {
    Self {
        inner: CreateTenantBuilder::new(name.into(),platform_fee_rate.into(),)
    }
}
    /// 口座名義（Payouts型アカウントの場合は必須）
pub fn bank_account_holder_name(mut self, bank_account_holder_name: impl Into<String>) -> Self {
    self.inner.bank_account_holder_name = Some(bank_account_holder_name.into());
    self
}
    /// 口座番号（Payouts型アカウントの場合は必須）
pub fn bank_account_number(mut self, bank_account_number: impl Into<String>) -> Self {
    self.inner.bank_account_number = Some(bank_account_number.into());
    self
}
    /// 預金種別（Payouts型アカウントの場合は必須）
pub fn bank_account_type(mut self, bank_account_type: impl Into<String>) -> Self {
    self.inner.bank_account_type = Some(bank_account_type.into());
    self
}
    /// 3桁の支店コード（Payouts型アカウントの場合は必須）
pub fn bank_branch_code(mut self, bank_branch_code: impl Into<String>) -> Self {
    self.inner.bank_branch_code = Some(bank_branch_code.into());
    self
}
    /// 4桁の銀行コード（Payouts型アカウントの場合は必須）
pub fn bank_code(mut self, bank_code: impl Into<String>) -> Self {
    self.inner.bank_code = Some(bank_code.into());
    self
}
        /// テナントIDとなる任意の文字列。一意にならないとエラーになります。また、未指定時は自動生成されます。.
pub fn id(mut self, id: impl Into<String>) -> Self {
    self.inner.id = Some(id.into());
    self
}
pub fn metadata(mut self, metadata: impl Into<payjp_shared::Metadata>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
    /// 最低入金額。デフォルトは1万円で下限は1000円。
pub fn minimum_transfer_amount(mut self, minimum_transfer_amount: impl Into<i64>) -> Self {
    self.inner.minimum_transfer_amount = Some(minimum_transfer_amount.into());
    self
}
        /// テナントのプラットフォーム利用料にPAY.JP決済手数料を含めるかどうか。デフォルトはfalse。作成時にのみ指定可能で、あとから変更はできません。.
pub fn payjp_fee_included(mut self, payjp_fee_included: impl Into<bool>) -> Self {
    self.inner.payjp_fee_included = Some(payjp_fee_included.into());
    self
}

}
    impl CreateTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for CreateTenant {
    type Output = payjp_core::Tenant;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Post, "/tenants").form(&self.inner)
}

}
    /// テナント情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveTenant {
 tenant: String,

}
impl RetrieveTenant {
    /// Construct a new `RetrieveTenant`.
pub fn new(tenant:impl Into<String>) -> Self {
    Self {
        tenant: tenant.into(),
    }
}

}
    impl RetrieveTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveTenant {
    type Output = payjp_core::Tenant;

    fn build(&self) -> RequestBuilder {
    let tenant = &self.tenant;
RequestBuilder::new(PayjpMethod::Get, format!("/tenants/{tenant}"))
}

}
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
 struct UpdateTenantBuilder {
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_holder_name: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_number: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_account_type: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_branch_code: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 bank_code: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 metadata: Option<payjp_shared::Metadata>,
#[serde(skip_serializing_if = "Option::is_none")]
 minimum_transfer_amount: Option<i64>,
#[serde(skip_serializing_if = "Option::is_none")]
 name: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
 platform_fee_rate: Option<f64>,

}
impl UpdateTenantBuilder {
     fn new() -> Self {
    Self {
        bank_account_holder_name: None,bank_account_number: None,bank_account_type: None,bank_branch_code: None,bank_code: None,metadata: None,minimum_transfer_amount: None,name: None,platform_fee_rate: None,
    }
}

}
        /// 生成したテナント情報を更新することができます。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct UpdateTenant {
 inner: UpdateTenantBuilder,
 tenant: String,

}
impl UpdateTenant {
    /// Construct a new `UpdateTenant`.
pub fn new(tenant:impl Into<String>) -> Self {
    Self {
        tenant: tenant.into(),inner: UpdateTenantBuilder::new()
    }
}
    /// 口座名義
pub fn bank_account_holder_name(mut self, bank_account_holder_name: impl Into<String>) -> Self {
    self.inner.bank_account_holder_name = Some(bank_account_holder_name.into());
    self
}
    /// 口座番号
pub fn bank_account_number(mut self, bank_account_number: impl Into<String>) -> Self {
    self.inner.bank_account_number = Some(bank_account_number.into());
    self
}
    /// 預金種別
pub fn bank_account_type(mut self, bank_account_type: impl Into<String>) -> Self {
    self.inner.bank_account_type = Some(bank_account_type.into());
    self
}
    /// 3桁の支店コード
pub fn bank_branch_code(mut self, bank_branch_code: impl Into<String>) -> Self {
    self.inner.bank_branch_code = Some(bank_branch_code.into());
    self
}
    /// 4桁の銀行コード
pub fn bank_code(mut self, bank_code: impl Into<String>) -> Self {
    self.inner.bank_code = Some(bank_code.into());
    self
}
pub fn metadata(mut self, metadata: impl Into<payjp_shared::Metadata>) -> Self {
    self.inner.metadata = Some(metadata.into());
    self
}
    /// 最低入金額。デフォルトは1万円で下限は1000円。
pub fn minimum_transfer_amount(mut self, minimum_transfer_amount: impl Into<i64>) -> Self {
    self.inner.minimum_transfer_amount = Some(minimum_transfer_amount.into());
    self
}
    /// テナント名
pub fn name(mut self, name: impl Into<String>) -> Self {
    self.inner.name = Some(name.into());
    self
}
        /// テナントのプラットフォーム利用料率(%)。小数点以下2桁まで入力可能。最大95%.
pub fn platform_fee_rate(mut self, platform_fee_rate: impl Into<f64>) -> Self {
    self.inner.platform_fee_rate = Some(platform_fee_rate.into());
    self
}

}
    impl UpdateTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for UpdateTenant {
    type Output = payjp_core::Tenant;

    fn build(&self) -> RequestBuilder {
    let tenant = &self.tenant;
RequestBuilder::new(PayjpMethod::Post, format!("/tenants/{tenant}")).form(&self.inner)
}

}
    /// 生成したテナント情報を削除します。
    ///
        /// 削除したテナントと同じIDのテナントをもう一度生成することができますが、削除したテナントとは別のテナントとして扱われます。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct DeleteTenant {
 tenant: String,

}
impl DeleteTenant {
    /// Construct a new `DeleteTenant`.
pub fn new(tenant:impl Into<String>) -> Self {
    Self {
        tenant: tenant.into(),
    }
}

}
    impl DeleteTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for DeleteTenant {
    type Output = payjp_shared::DeleteResponse;

    fn build(&self) -> RequestBuilder {
    let tenant = &self.tenant;
RequestBuilder::new(PayjpMethod::Delete, format!("/tenants/{tenant}"))
}

}
        /// (Marketplace型アカウントのみ利用可能)テナントの審査申請ページのURLを作成します。.
    ///
        /// テストモードの場合、実際の審査は行われず情報が保存されない為、常に新規申請になります。.
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct ApplicationUrlsTenant {
 tenant: String,

}
impl ApplicationUrlsTenant {
    /// Construct a new `ApplicationUrlsTenant`.
pub fn new(tenant:impl Into<String>) -> Self {
    Self {
        tenant: tenant.into(),
    }
}

}
    impl ApplicationUrlsTenant {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for ApplicationUrlsTenant {
    type Output = ApplicationUrlsTenantReturned;

    fn build(&self) -> RequestBuilder {
    let tenant = &self.tenant;
RequestBuilder::new(PayjpMethod::Post, format!("/tenants/{tenant}/application_urls"))
}

}
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ApplicationUrlsTenantReturned {
    /// application_urlの使用期限のタイムスタンプ。発行から5分
pub expires: i64,
    /// 固定文字列
pub object: ApplicationUrlsTenantReturnedObject,
    /// テナントの審査申請URL
pub url: String,

}
#[doc(hidden)]
pub struct ApplicationUrlsTenantReturnedBuilder {
    expires: Option<i64>,
object: Option<ApplicationUrlsTenantReturnedObject>,
url: Option<String>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for ApplicationUrlsTenantReturned {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<ApplicationUrlsTenantReturned>,
    builder: ApplicationUrlsTenantReturnedBuilder,
}

impl Visitor for Place<ApplicationUrlsTenantReturned> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ApplicationUrlsTenantReturnedBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ApplicationUrlsTenantReturnedBuilder {
    type Out = ApplicationUrlsTenantReturned;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "expires" => Deserialize::begin(&mut self.expires),
"object" => Deserialize::begin(&mut self.object),
"url" => Deserialize::begin(&mut self.url),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            expires: Deserialize::default(),
object: Deserialize::default(),
url: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(expires),
Some(object),
Some(url),
) = (self.expires,
self.object,
self.url.take(),
) else {
            return None;
        };
        Some(Self::Out { expires,object,url })
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

impl ObjectDeser for ApplicationUrlsTenantReturned {
    type Builder = ApplicationUrlsTenantReturnedBuilder;
}

impl FromValueOpt for ApplicationUrlsTenantReturned {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ApplicationUrlsTenantReturnedBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "expires" => b.expires = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"url" => b.url = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 固定文字列
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ApplicationUrlsTenantReturnedObject {
ApplicationUrl,

}
impl ApplicationUrlsTenantReturnedObject {
    pub fn as_str(self) -> &'static str {
        use ApplicationUrlsTenantReturnedObject::*;
        match self {
ApplicationUrl => "application_url",

        }
    }
}

impl std::str::FromStr for ApplicationUrlsTenantReturnedObject {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApplicationUrlsTenantReturnedObject::*;
        match s {
    "application_url" => Ok(ApplicationUrl),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ApplicationUrlsTenantReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ApplicationUrlsTenantReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ApplicationUrlsTenantReturnedObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ApplicationUrlsTenantReturnedObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ApplicationUrlsTenantReturnedObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ApplicationUrlsTenantReturnedObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ApplicationUrlsTenantReturnedObject);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ApplicationUrlsTenantReturnedObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ApplicationUrlsTenantReturnedObject"))
    }
}

