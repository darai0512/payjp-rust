/// tenantオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Tenant {
    /// 口座名義
pub bank_account_holder_name: Option<String>,
    /// 口座番号
pub bank_account_number: Option<String>,
    /// 口座状態。pending:未確認, success:入金確認済み, failed:入金不可能
pub bank_account_status: Option<TenantBankAccountStatus>,
    /// 預金種別
pub bank_account_type: Option<String>,
    /// 3桁の支店コード
pub bank_branch_code: Option<String>,
    /// 4桁の銀行コード
pub bank_code: Option<String>,
    /// 定期課金作成時のUTCタイムスタンプ
pub created: i64,
    /// 対応通貨のリスト(文字列)
pub currencies_supported: Option<Vec<String>>,
    /// 3文字のISOコード(現状 “jpy” のみサポート)
pub default_currency: Option<String>,
        /// `ten_`で始まる自動生成された一意な文字列、または作成時に指定した任意の文字列.
pub id: payjp_core::TenantId,
    /// 本番環境かどうか
pub livemode: bool,
pub metadata: Option<payjp_shared::Metadata>,
    /// 最低入金額
pub minimum_transfer_amount: i64,
    /// "tenant"の固定文字列
pub object: String,
    /// テナントのプラットフォーム利用料にPAY.JP決済手数料を含めるかどうか
pub payjp_fee_included: bool,
    /// テナントのプラットフォーム利用料率(%)
pub platform_fee_rate: String,
    /// 申請情報を提出済のブランドの各種情報
pub reviewed_brands: Option<Vec<TenantReviewedBrands>>,

}
#[doc(hidden)]
pub struct TenantBuilder {
    bank_account_holder_name: Option<Option<String>>,
bank_account_number: Option<Option<String>>,
bank_account_status: Option<Option<TenantBankAccountStatus>>,
bank_account_type: Option<Option<String>>,
bank_branch_code: Option<Option<String>>,
bank_code: Option<Option<String>>,
created: Option<i64>,
currencies_supported: Option<Option<Vec<String>>>,
default_currency: Option<Option<String>>,
id: Option<payjp_core::TenantId>,
livemode: Option<bool>,
metadata: Option<Option<payjp_shared::Metadata>>,
minimum_transfer_amount: Option<i64>,
object: Option<String>,
payjp_fee_included: Option<bool>,
platform_fee_rate: Option<String>,
reviewed_brands: Option<Option<Vec<TenantReviewedBrands>>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Tenant {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Tenant>,
    builder: TenantBuilder,
}

impl Visitor for Place<Tenant> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TenantBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for TenantBuilder {
    type Out = Tenant;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "bank_account_holder_name" => Deserialize::begin(&mut self.bank_account_holder_name),
"bank_account_number" => Deserialize::begin(&mut self.bank_account_number),
"bank_account_status" => Deserialize::begin(&mut self.bank_account_status),
"bank_account_type" => Deserialize::begin(&mut self.bank_account_type),
"bank_branch_code" => Deserialize::begin(&mut self.bank_branch_code),
"bank_code" => Deserialize::begin(&mut self.bank_code),
"created" => Deserialize::begin(&mut self.created),
"currencies_supported" => Deserialize::begin(&mut self.currencies_supported),
"default_currency" => Deserialize::begin(&mut self.default_currency),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"minimum_transfer_amount" => Deserialize::begin(&mut self.minimum_transfer_amount),
"object" => Deserialize::begin(&mut self.object),
"payjp_fee_included" => Deserialize::begin(&mut self.payjp_fee_included),
"platform_fee_rate" => Deserialize::begin(&mut self.platform_fee_rate),
"reviewed_brands" => Deserialize::begin(&mut self.reviewed_brands),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            bank_account_holder_name: Deserialize::default(),
bank_account_number: Deserialize::default(),
bank_account_status: Deserialize::default(),
bank_account_type: Deserialize::default(),
bank_branch_code: Deserialize::default(),
bank_code: Deserialize::default(),
created: Deserialize::default(),
currencies_supported: Deserialize::default(),
default_currency: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
minimum_transfer_amount: Deserialize::default(),
object: Deserialize::default(),
payjp_fee_included: Deserialize::default(),
platform_fee_rate: Deserialize::default(),
reviewed_brands: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(bank_account_holder_name),
Some(bank_account_number),
Some(bank_account_status),
Some(bank_account_type),
Some(bank_branch_code),
Some(bank_code),
Some(created),
Some(currencies_supported),
Some(default_currency),
Some(id),
Some(livemode),
Some(metadata),
Some(minimum_transfer_amount),
Some(object),
Some(payjp_fee_included),
Some(platform_fee_rate),
Some(reviewed_brands),
) = (self.bank_account_holder_name.take(),
self.bank_account_number.take(),
self.bank_account_status,
self.bank_account_type.take(),
self.bank_branch_code.take(),
self.bank_code.take(),
self.created,
self.currencies_supported.take(),
self.default_currency.take(),
self.id.take(),
self.livemode,
self.metadata,
self.minimum_transfer_amount,
self.object.take(),
self.payjp_fee_included,
self.platform_fee_rate.take(),
self.reviewed_brands.take(),
) else {
            return None;
        };
        Some(Self::Out { bank_account_holder_name,bank_account_number,bank_account_status,bank_account_type,bank_branch_code,bank_code,created,currencies_supported,default_currency,id,livemode,metadata,minimum_transfer_amount,object,payjp_fee_included,platform_fee_rate,reviewed_brands })
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

impl ObjectDeser for Tenant {
    type Builder = TenantBuilder;
}

impl FromValueOpt for Tenant {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = TenantBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "bank_account_holder_name" => b.bank_account_holder_name = FromValueOpt::from_value(v),
"bank_account_number" => b.bank_account_number = FromValueOpt::from_value(v),
"bank_account_status" => b.bank_account_status = FromValueOpt::from_value(v),
"bank_account_type" => b.bank_account_type = FromValueOpt::from_value(v),
"bank_branch_code" => b.bank_branch_code = FromValueOpt::from_value(v),
"bank_code" => b.bank_code = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"currencies_supported" => b.currencies_supported = FromValueOpt::from_value(v),
"default_currency" => b.default_currency = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"minimum_transfer_amount" => b.minimum_transfer_amount = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"payjp_fee_included" => b.payjp_fee_included = FromValueOpt::from_value(v),
"platform_fee_rate" => b.platform_fee_rate = FromValueOpt::from_value(v),
"reviewed_brands" => b.reviewed_brands = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 口座状態。pending:未確認, success:入金確認済み, failed:入金不可能
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum TenantBankAccountStatus {
Pending,
Success,
Failed,

}
impl TenantBankAccountStatus {
    pub fn as_str(self) -> &'static str {
        use TenantBankAccountStatus::*;
        match self {
Pending => "pending",
Success => "success",
Failed => "failed",

        }
    }
}

impl std::str::FromStr for TenantBankAccountStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TenantBankAccountStatus::*;
        match s {
    "pending" => Ok(Pending),
"success" => Ok(Success),
"failed" => Ok(Failed),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for TenantBankAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TenantBankAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TenantBankAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TenantBankAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TenantBankAccountStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TenantBankAccountStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(TenantBankAccountStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TenantBankAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TenantBankAccountStatus"))
    }
}
/// 申請情報を提出済のブランドの各種情報
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TenantReviewedBrands {
    /// 利用可能開始時刻のタイムスタンプ
pub available_date: Option<payjp_types::Timestamp>,
    /// ブランド名
pub brand: String,
    /// 審査ステータス
pub status: TenantReviewedBrandsStatus,

}
#[doc(hidden)]
pub struct TenantReviewedBrandsBuilder {
    available_date: Option<Option<payjp_types::Timestamp>>,
brand: Option<String>,
status: Option<TenantReviewedBrandsStatus>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for TenantReviewedBrands {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<TenantReviewedBrands>,
    builder: TenantReviewedBrandsBuilder,
}

impl Visitor for Place<TenantReviewedBrands> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TenantReviewedBrandsBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for TenantReviewedBrandsBuilder {
    type Out = TenantReviewedBrands;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "available_date" => Deserialize::begin(&mut self.available_date),
"brand" => Deserialize::begin(&mut self.brand),
"status" => Deserialize::begin(&mut self.status),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            available_date: Deserialize::default(),
brand: Deserialize::default(),
status: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(available_date),
Some(brand),
Some(status),
) = (self.available_date,
self.brand.take(),
self.status,
) else {
            return None;
        };
        Some(Self::Out { available_date,brand,status })
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

impl ObjectDeser for TenantReviewedBrands {
    type Builder = TenantReviewedBrandsBuilder;
}

impl FromValueOpt for TenantReviewedBrands {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = TenantReviewedBrandsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "available_date" => b.available_date = FromValueOpt::from_value(v),
"brand" => b.brand = FromValueOpt::from_value(v),
"status" => b.status = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 審査ステータス
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum TenantReviewedBrandsStatus {
Passed,
InReview,
Declined,

}
impl TenantReviewedBrandsStatus {
    pub fn as_str(self) -> &'static str {
        use TenantReviewedBrandsStatus::*;
        match self {
Passed => "passed",
InReview => "in_review",
Declined => "declined",

        }
    }
}

impl std::str::FromStr for TenantReviewedBrandsStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TenantReviewedBrandsStatus::*;
        match s {
    "passed" => Ok(Passed),
"in_review" => Ok(InReview),
"declined" => Ok(Declined),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for TenantReviewedBrandsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TenantReviewedBrandsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TenantReviewedBrandsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TenantReviewedBrandsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TenantReviewedBrandsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TenantReviewedBrandsStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(TenantReviewedBrandsStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TenantReviewedBrandsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TenantReviewedBrandsStatus"))
    }
}
impl payjp_types::Object for Tenant {
    type Id = payjp_core::TenantId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(TenantId);
