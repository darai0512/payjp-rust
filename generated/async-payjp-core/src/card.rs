/// クレジットカードの情報を表すcardオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Card {
    /// 請求先住所（市区町村）
pub address_city: Option<String>,
    /// 請求先住所（番地）
pub address_line1: Option<String>,
    /// 請求先住所（建物名）
pub address_line2: Option<String>,
    /// 請求先住所（都道府県）
pub address_state: Option<String>,
    /// 請求先住所（郵便番号）
pub address_zip: Option<String>,
    /// 郵便番号の検証結果
pub address_zip_check: Option<CardAddressZipCheck>,
    /// カードブランド名
pub brand: CardBrand,
    /// 2桁のISOコード(e.g. JP)
pub country: Option<String>,
    /// カード作成時のタイムスタンプ
pub created: i64,
    /// このカードを保有する顧客ID
pub customer: Option<String>,
    /// セキュリティコードの検証結果
pub cvc_check: Option<CardCvcCheck>,
    /// メールアドレス
        /// 2024年8月以降、3Dセキュア認証の際にphoneまたはemailのデータ入力が求められます。.
pub email: Option<String>,
    /// 有効期限月
pub exp_month: i64,
    /// 有効期限年
pub exp_year: i64,
    /// このクレジットカード番号に紐づく値。
        /// 同一番号のカードからは同一の値が生成されることが保証されており、 トークン化の度にトークンIDは変わりますが、この値は変わりません。.
pub fingerprint: String,
    /// car_で始まり一意なオブジェクトを示す文字列
pub id: payjp_core::CardId,
    /// カード番号の下四桁
pub last4: String,
    /// 本番環境かどうか
pub livemode: bool,
pub metadata: Option<payjp_shared::Metadata>,
    /// カード保有者名
pub name: Option<String>,
    /// E.164形式の電話番号 (e.g. 090-0123-4567（日本） => "+819001234567")
        /// 2024年8月以降、3Dセキュア認証の際にphoneまたはemailのデータ入力が求められます。.
pub phone: Option<String>,
    /// 3Dセキュアの利用状況
pub three_d_secure_status: Option<CardThreeDSecureStatus>,

}
#[doc(hidden)]
pub struct CardBuilder {
    address_city: Option<Option<String>>,
address_line1: Option<Option<String>>,
address_line2: Option<Option<String>>,
address_state: Option<Option<String>>,
address_zip: Option<Option<String>>,
address_zip_check: Option<Option<CardAddressZipCheck>>,
brand: Option<CardBrand>,
country: Option<Option<String>>,
created: Option<i64>,
customer: Option<Option<String>>,
cvc_check: Option<Option<CardCvcCheck>>,
email: Option<Option<String>>,
exp_month: Option<i64>,
exp_year: Option<i64>,
fingerprint: Option<String>,
id: Option<payjp_core::CardId>,
last4: Option<String>,
livemode: Option<bool>,
metadata: Option<Option<payjp_shared::Metadata>>,
name: Option<Option<String>>,
phone: Option<Option<String>>,
three_d_secure_status: Option<Option<CardThreeDSecureStatus>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Card {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Card>,
    builder: CardBuilder,
}

impl Visitor for Place<Card> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CardBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for CardBuilder {
    type Out = Card;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "address_city" => Deserialize::begin(&mut self.address_city),
"address_line1" => Deserialize::begin(&mut self.address_line1),
"address_line2" => Deserialize::begin(&mut self.address_line2),
"address_state" => Deserialize::begin(&mut self.address_state),
"address_zip" => Deserialize::begin(&mut self.address_zip),
"address_zip_check" => Deserialize::begin(&mut self.address_zip_check),
"brand" => Deserialize::begin(&mut self.brand),
"country" => Deserialize::begin(&mut self.country),
"created" => Deserialize::begin(&mut self.created),
"customer" => Deserialize::begin(&mut self.customer),
"cvc_check" => Deserialize::begin(&mut self.cvc_check),
"email" => Deserialize::begin(&mut self.email),
"exp_month" => Deserialize::begin(&mut self.exp_month),
"exp_year" => Deserialize::begin(&mut self.exp_year),
"fingerprint" => Deserialize::begin(&mut self.fingerprint),
"id" => Deserialize::begin(&mut self.id),
"last4" => Deserialize::begin(&mut self.last4),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"name" => Deserialize::begin(&mut self.name),
"phone" => Deserialize::begin(&mut self.phone),
"three_d_secure_status" => Deserialize::begin(&mut self.three_d_secure_status),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            address_city: Deserialize::default(),
address_line1: Deserialize::default(),
address_line2: Deserialize::default(),
address_state: Deserialize::default(),
address_zip: Deserialize::default(),
address_zip_check: Deserialize::default(),
brand: Deserialize::default(),
country: Deserialize::default(),
created: Deserialize::default(),
customer: Deserialize::default(),
cvc_check: Deserialize::default(),
email: Deserialize::default(),
exp_month: Deserialize::default(),
exp_year: Deserialize::default(),
fingerprint: Deserialize::default(),
id: Deserialize::default(),
last4: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
name: Deserialize::default(),
phone: Deserialize::default(),
three_d_secure_status: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(address_city),
Some(address_line1),
Some(address_line2),
Some(address_state),
Some(address_zip),
Some(address_zip_check),
Some(brand),
Some(country),
Some(created),
Some(customer),
Some(cvc_check),
Some(email),
Some(exp_month),
Some(exp_year),
Some(fingerprint),
Some(id),
Some(last4),
Some(livemode),
Some(metadata),
Some(name),
Some(phone),
Some(three_d_secure_status),
) = (self.address_city.take(),
self.address_line1.take(),
self.address_line2.take(),
self.address_state.take(),
self.address_zip.take(),
self.address_zip_check,
self.brand,
self.country.take(),
self.created,
self.customer.take(),
self.cvc_check,
self.email.take(),
self.exp_month,
self.exp_year,
self.fingerprint.take(),
self.id.take(),
self.last4.take(),
self.livemode,
self.metadata,
self.name.take(),
self.phone.take(),
self.three_d_secure_status,
) else {
            return None;
        };
        Some(Self::Out { address_city,address_line1,address_line2,address_state,address_zip,address_zip_check,brand,country,created,customer,cvc_check,email,exp_month,exp_year,fingerprint,id,last4,livemode,metadata,name,phone,three_d_secure_status })
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

impl ObjectDeser for Card {
    type Builder = CardBuilder;
}

impl FromValueOpt for Card {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CardBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "address_city" => b.address_city = FromValueOpt::from_value(v),
"address_line1" => b.address_line1 = FromValueOpt::from_value(v),
"address_line2" => b.address_line2 = FromValueOpt::from_value(v),
"address_state" => b.address_state = FromValueOpt::from_value(v),
"address_zip" => b.address_zip = FromValueOpt::from_value(v),
"address_zip_check" => b.address_zip_check = FromValueOpt::from_value(v),
"brand" => b.brand = FromValueOpt::from_value(v),
"country" => b.country = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"customer" => b.customer = FromValueOpt::from_value(v),
"cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
"email" => b.email = FromValueOpt::from_value(v),
"exp_month" => b.exp_month = FromValueOpt::from_value(v),
"exp_year" => b.exp_year = FromValueOpt::from_value(v),
"fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"last4" => b.last4 = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"name" => b.name = FromValueOpt::from_value(v),
"phone" => b.phone = FromValueOpt::from_value(v),
"three_d_secure_status" => b.three_d_secure_status = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Card {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Card", 23)?;
        s.serialize_field("address_city", &self.address_city)?;
s.serialize_field("address_line1", &self.address_line1)?;
s.serialize_field("address_line2", &self.address_line2)?;
s.serialize_field("address_state", &self.address_state)?;
s.serialize_field("address_zip", &self.address_zip)?;
s.serialize_field("address_zip_check", &self.address_zip_check)?;
s.serialize_field("brand", &self.brand)?;
s.serialize_field("country", &self.country)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("customer", &self.customer)?;
s.serialize_field("cvc_check", &self.cvc_check)?;
s.serialize_field("email", &self.email)?;
s.serialize_field("exp_month", &self.exp_month)?;
s.serialize_field("exp_year", &self.exp_year)?;
s.serialize_field("fingerprint", &self.fingerprint)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("last4", &self.last4)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("metadata", &self.metadata)?;
s.serialize_field("name", &self.name)?;
s.serialize_field("phone", &self.phone)?;
s.serialize_field("three_d_secure_status", &self.three_d_secure_status)?;

        s.serialize_field("object", "card")?;
        s.end()
    }
}
/// 郵便番号の検証結果
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum CardAddressZipCheck {
Passed,
Failed,
Unchecked,

}
impl CardAddressZipCheck {
    pub fn as_str(self) -> &'static str {
        use CardAddressZipCheck::*;
        match self {
Passed => "passed",
Failed => "failed",
Unchecked => "unchecked",

        }
    }
}

impl std::str::FromStr for CardAddressZipCheck {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardAddressZipCheck::*;
        match s {
    "passed" => Ok(Passed),
"failed" => Ok(Failed),
"unchecked" => Ok(Unchecked),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for CardAddressZipCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardAddressZipCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CardAddressZipCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardAddressZipCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardAddressZipCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardAddressZipCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(CardAddressZipCheck);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardAddressZipCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardAddressZipCheck"))
    }
}
/// カードブランド名
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum CardBrand {
Visa,
MasterCard,
Jcb,
AmericanExpress,
DinersClub,
Discover,

}
impl CardBrand {
    pub fn as_str(self) -> &'static str {
        use CardBrand::*;
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

impl std::str::FromStr for CardBrand {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardBrand::*;
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
impl std::fmt::Display for CardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CardBrand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardBrand {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardBrand> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardBrand::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(CardBrand);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardBrand {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardBrand"))
    }
}
/// セキュリティコードの検証結果
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum CardCvcCheck {
Passed,
Failed,
Unchecked,

}
impl CardCvcCheck {
    pub fn as_str(self) -> &'static str {
        use CardCvcCheck::*;
        match self {
Passed => "passed",
Failed => "failed",
Unchecked => "unchecked",

        }
    }
}

impl std::str::FromStr for CardCvcCheck {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardCvcCheck::*;
        match s {
    "passed" => Ok(Passed),
"failed" => Ok(Failed),
"unchecked" => Ok(Unchecked),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for CardCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CardCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardCvcCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardCvcCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardCvcCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(CardCvcCheck);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardCvcCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardCvcCheck"))
    }
}
/// 3Dセキュアの利用状況
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum CardThreeDSecureStatus {
Available,
Unavailable,

}
impl CardThreeDSecureStatus {
    pub fn as_str(self) -> &'static str {
        use CardThreeDSecureStatus::*;
        match self {
Available => "available",
Unavailable => "unavailable",

        }
    }
}

impl std::str::FromStr for CardThreeDSecureStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardThreeDSecureStatus::*;
        match s {
    "available" => Ok(Available),
"unavailable" => Ok(Unavailable),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for CardThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CardThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CardThreeDSecureStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardThreeDSecureStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardThreeDSecureStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardThreeDSecureStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(CardThreeDSecureStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardThreeDSecureStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardThreeDSecureStatus"))
    }
}
impl payjp_types::Object for Card {
    type Id = payjp_core::CardId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(CardId);
