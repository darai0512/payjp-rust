/// Merchantオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Merchant {
    /// 入金先銀行口座情報が設定済みかどうか
pub bank_enabled: bool,
    /// 本番環境で利用可能なカードブランドのリスト
pub brands_accepted: Vec<MerchantBrandsAccepted>,
    /// 業務形態
pub business_type: Option<String>,
    /// 支払い方法種別のリスト
pub charge_type: Option<Vec<String>>,
    /// 所在国
pub country: String,
    /// 登録日時
pub created: i64,
    /// 対応通貨のリスト
pub currencies_supported: Vec<MerchantCurrenciesSupported>,
    /// 3文字のISOコード（現状 "jpy" のみサポート）
pub default_currency: MerchantDefaultCurrency,
    /// 本番環境申請情報が提出済みかどうか
pub details_submitted: bool,
    /// acct_mch_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::MerchantId,
    /// 本番環境が許可された日時のUTCタイムスタンプ
pub livemode_activated_at: Option<i64>,
    /// 本番環境が有効かどうか
pub livemode_enabled: bool,
    /// 販売商品名
pub product_name: Option<String>,
    /// 販売商品の種類リスト
pub product_type: Option<Vec<String>>,
    /// 申請対象のサイトがオープン済みかどうか
pub site_published: Option<bool>,

}
#[doc(hidden)]
pub struct MerchantBuilder {
    bank_enabled: Option<bool>,
brands_accepted: Option<Vec<MerchantBrandsAccepted>>,
business_type: Option<Option<String>>,
charge_type: Option<Option<Vec<String>>>,
country: Option<String>,
created: Option<i64>,
currencies_supported: Option<Vec<MerchantCurrenciesSupported>>,
default_currency: Option<MerchantDefaultCurrency>,
details_submitted: Option<bool>,
id: Option<payjp_core::MerchantId>,
livemode_activated_at: Option<Option<i64>>,
livemode_enabled: Option<bool>,
product_name: Option<Option<String>>,
product_type: Option<Option<Vec<String>>>,
site_published: Option<Option<bool>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Merchant {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Merchant>,
    builder: MerchantBuilder,
}

impl Visitor for Place<Merchant> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: MerchantBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for MerchantBuilder {
    type Out = Merchant;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "bank_enabled" => Deserialize::begin(&mut self.bank_enabled),
"brands_accepted" => Deserialize::begin(&mut self.brands_accepted),
"business_type" => Deserialize::begin(&mut self.business_type),
"charge_type" => Deserialize::begin(&mut self.charge_type),
"country" => Deserialize::begin(&mut self.country),
"created" => Deserialize::begin(&mut self.created),
"currencies_supported" => Deserialize::begin(&mut self.currencies_supported),
"default_currency" => Deserialize::begin(&mut self.default_currency),
"details_submitted" => Deserialize::begin(&mut self.details_submitted),
"id" => Deserialize::begin(&mut self.id),
"livemode_activated_at" => Deserialize::begin(&mut self.livemode_activated_at),
"livemode_enabled" => Deserialize::begin(&mut self.livemode_enabled),
"product_name" => Deserialize::begin(&mut self.product_name),
"product_type" => Deserialize::begin(&mut self.product_type),
"site_published" => Deserialize::begin(&mut self.site_published),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            bank_enabled: Deserialize::default(),
brands_accepted: Deserialize::default(),
business_type: Deserialize::default(),
charge_type: Deserialize::default(),
country: Deserialize::default(),
created: Deserialize::default(),
currencies_supported: Deserialize::default(),
default_currency: Deserialize::default(),
details_submitted: Deserialize::default(),
id: Deserialize::default(),
livemode_activated_at: Deserialize::default(),
livemode_enabled: Deserialize::default(),
product_name: Deserialize::default(),
product_type: Deserialize::default(),
site_published: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(bank_enabled),
Some(brands_accepted),
Some(business_type),
Some(charge_type),
Some(country),
Some(created),
Some(currencies_supported),
Some(default_currency),
Some(details_submitted),
Some(id),
Some(livemode_activated_at),
Some(livemode_enabled),
Some(product_name),
Some(product_type),
Some(site_published),
) = (self.bank_enabled,
self.brands_accepted.take(),
self.business_type.take(),
self.charge_type.take(),
self.country.take(),
self.created,
self.currencies_supported.take(),
self.default_currency,
self.details_submitted,
self.id.take(),
self.livemode_activated_at,
self.livemode_enabled,
self.product_name.take(),
self.product_type.take(),
self.site_published,
) else {
            return None;
        };
        Some(Self::Out { bank_enabled,brands_accepted,business_type,charge_type,country,created,currencies_supported,default_currency,details_submitted,id,livemode_activated_at,livemode_enabled,product_name,product_type,site_published })
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

impl ObjectDeser for Merchant {
    type Builder = MerchantBuilder;
}

impl FromValueOpt for Merchant {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = MerchantBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "bank_enabled" => b.bank_enabled = FromValueOpt::from_value(v),
"brands_accepted" => b.brands_accepted = FromValueOpt::from_value(v),
"business_type" => b.business_type = FromValueOpt::from_value(v),
"charge_type" => b.charge_type = FromValueOpt::from_value(v),
"country" => b.country = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"currencies_supported" => b.currencies_supported = FromValueOpt::from_value(v),
"default_currency" => b.default_currency = FromValueOpt::from_value(v),
"details_submitted" => b.details_submitted = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode_activated_at" => b.livemode_activated_at = FromValueOpt::from_value(v),
"livemode_enabled" => b.livemode_enabled = FromValueOpt::from_value(v),
"product_name" => b.product_name = FromValueOpt::from_value(v),
"product_type" => b.product_type = FromValueOpt::from_value(v),
"site_published" => b.site_published = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Merchant {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Merchant", 16)?;
        s.serialize_field("bank_enabled", &self.bank_enabled)?;
s.serialize_field("brands_accepted", &self.brands_accepted)?;
s.serialize_field("business_type", &self.business_type)?;
s.serialize_field("charge_type", &self.charge_type)?;
s.serialize_field("country", &self.country)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("currencies_supported", &self.currencies_supported)?;
s.serialize_field("default_currency", &self.default_currency)?;
s.serialize_field("details_submitted", &self.details_submitted)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode_activated_at", &self.livemode_activated_at)?;
s.serialize_field("livemode_enabled", &self.livemode_enabled)?;
s.serialize_field("product_name", &self.product_name)?;
s.serialize_field("product_type", &self.product_type)?;
s.serialize_field("site_published", &self.site_published)?;

        s.serialize_field("object", "merchant")?;
        s.end()
    }
}
/// 本番環境で利用可能なカードブランドのリスト
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum MerchantBrandsAccepted {
Visa,
MasterCard,
Jcb,
AmericanExpress,
DinersClub,
Discover,

}
impl MerchantBrandsAccepted {
    pub fn as_str(self) -> &'static str {
        use MerchantBrandsAccepted::*;
        match self {
Visa => "Visa",
MasterCard => "MasterCard",
Jcb => "JCB",
AmericanExpress => "American Express",
DinersClub => "Diners Club",
Discover => "Discover",

        }
    }
}

impl std::str::FromStr for MerchantBrandsAccepted {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MerchantBrandsAccepted::*;
        match s {
    "Visa" => Ok(Visa),
"MasterCard" => Ok(MasterCard),
"JCB" => Ok(Jcb),
"American Express" => Ok(AmericanExpress),
"Diners Club" => Ok(DinersClub),
"Discover" => Ok(Discover),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for MerchantBrandsAccepted {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MerchantBrandsAccepted {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MerchantBrandsAccepted {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MerchantBrandsAccepted {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MerchantBrandsAccepted> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MerchantBrandsAccepted::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(MerchantBrandsAccepted);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MerchantBrandsAccepted {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MerchantBrandsAccepted"))
    }
}
/// 対応通貨のリスト
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum MerchantCurrenciesSupported {
Jpy,

}
impl MerchantCurrenciesSupported {
    pub fn as_str(self) -> &'static str {
        use MerchantCurrenciesSupported::*;
        match self {
Jpy => "jpy",

        }
    }
}

impl std::str::FromStr for MerchantCurrenciesSupported {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MerchantCurrenciesSupported::*;
        match s {
    "jpy" => Ok(Jpy),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for MerchantCurrenciesSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MerchantCurrenciesSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MerchantCurrenciesSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MerchantCurrenciesSupported {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MerchantCurrenciesSupported> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MerchantCurrenciesSupported::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(MerchantCurrenciesSupported);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MerchantCurrenciesSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MerchantCurrenciesSupported"))
    }
}
/// 3文字のISOコード（現状 "jpy" のみサポート）
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum MerchantDefaultCurrency {
Jpy,

}
impl MerchantDefaultCurrency {
    pub fn as_str(self) -> &'static str {
        use MerchantDefaultCurrency::*;
        match self {
Jpy => "jpy",

        }
    }
}

impl std::str::FromStr for MerchantDefaultCurrency {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MerchantDefaultCurrency::*;
        match s {
    "jpy" => Ok(Jpy),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for MerchantDefaultCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MerchantDefaultCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MerchantDefaultCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MerchantDefaultCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MerchantDefaultCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MerchantDefaultCurrency::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(MerchantDefaultCurrency);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MerchantDefaultCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MerchantDefaultCurrency"))
    }
}
impl payjp_types::Object for Merchant {
    type Id = payjp_core::MerchantId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(MerchantId);
