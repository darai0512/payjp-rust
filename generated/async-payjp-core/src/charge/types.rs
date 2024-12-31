/// chargeオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Charge {
    /// 支払い額
pub amount: i64,
    /// この支払いに対しての返金額
pub amount_refunded: Option<i64>,
    /// 支払い処理を確定しているかどうか
pub captured: bool,
    /// 支払い処理確定時のUTCタイムスタンプ
pub captured_at: Option<i64>,
pub card: payjp_core::Card,
    /// この支払い作成時のUTCタイムスタンプ
pub created: i64,
    /// 3文字のISOコード(現状 "jpy" のみサポート)
pub currency: ChargeCurrency,
    /// 顧客ID
pub customer: Option<String>,
    /// 概要
pub description: Option<String>,
    /// 認証状態が自動的に失効される日時のタイムスタンプ
pub expired_at: Option<i64>,
    /// 失敗した支払いのエラーコード
pub failure_code: Option<String>,
    /// 失敗した支払いの説明
pub failure_message: Option<String>,
    /// 決済手数料率
pub fee_rate: Option<String>,
    /// ch_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::ChargeId,
    /// 本番環境かどうか
pub livemode: bool,
    /// キーバリューの任意データ
#[cfg_attr(feature = "deserialize", serde(with = "payjp_types::with_serde_json_opt"))]
pub metadata: Option<miniserde::json::Value>,
    /// 認証処理が成功しているかどうか。
pub paid: bool,
    /// [PAY.JP Platform](#platform-api) のみ
    ///
    /// プラットフォーマーに振り分けられる入金金額。
pub platform_fee: Option<i64>,
    /// [PAY.JP Platform](#platform-api)のみ
    ///
        /// [テナント作成](#テナントを作成)時に指定したプラットフォーム利用料率(%).
pub platform_fee_rate: Option<String>,
    /// 返金理由
pub refund_reason: Option<String>,
    /// 返金済みかどうか
pub refunded: Option<bool>,
    /// sub_から始まる定期課金のID
pub subscription: Option<String>,
    /// [PAY.JP Platform](#platform-api)のみ
    ///
    /// テナントID
pub tenant: Option<String>,
        /// [入金管理オブジェクトの刷新に伴い、2024/06/01以降に提供されます。](https://pay.jp/docs/migrate-transfer).
    ///
    /// この支払いが関連付けられたTermオブジェクトのID
pub term_id: Option<String>,
    /// 3Dセキュアの実施状況
pub three_d_secure_status: Option<ChargeThreeDSecureStatus>,
    /// [PAY.JP Platform](#platform-api)のみ
    ///
    /// プラットフォーム利用料総額
pub total_platform_fee: Option<i64>,

}
#[doc(hidden)]
pub struct ChargeBuilder {
    amount: Option<i64>,
amount_refunded: Option<Option<i64>>,
captured: Option<bool>,
captured_at: Option<Option<i64>>,
card: Option<payjp_core::Card>,
created: Option<i64>,
currency: Option<ChargeCurrency>,
customer: Option<Option<String>>,
description: Option<Option<String>>,
expired_at: Option<Option<i64>>,
failure_code: Option<Option<String>>,
failure_message: Option<Option<String>>,
fee_rate: Option<Option<String>>,
id: Option<payjp_core::ChargeId>,
livemode: Option<bool>,
metadata: Option<Option<miniserde::json::Value>>,
paid: Option<bool>,
platform_fee: Option<Option<i64>>,
platform_fee_rate: Option<Option<String>>,
refund_reason: Option<Option<String>>,
refunded: Option<Option<bool>>,
subscription: Option<Option<String>>,
tenant: Option<Option<String>>,
term_id: Option<Option<String>>,
three_d_secure_status: Option<Option<ChargeThreeDSecureStatus>>,
total_platform_fee: Option<Option<i64>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Charge {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Charge>,
    builder: ChargeBuilder,
}

impl Visitor for Place<Charge> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ChargeBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ChargeBuilder {
    type Out = Charge;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "amount" => Deserialize::begin(&mut self.amount),
"amount_refunded" => Deserialize::begin(&mut self.amount_refunded),
"captured" => Deserialize::begin(&mut self.captured),
"captured_at" => Deserialize::begin(&mut self.captured_at),
"card" => Deserialize::begin(&mut self.card),
"created" => Deserialize::begin(&mut self.created),
"currency" => Deserialize::begin(&mut self.currency),
"customer" => Deserialize::begin(&mut self.customer),
"description" => Deserialize::begin(&mut self.description),
"expired_at" => Deserialize::begin(&mut self.expired_at),
"failure_code" => Deserialize::begin(&mut self.failure_code),
"failure_message" => Deserialize::begin(&mut self.failure_message),
"fee_rate" => Deserialize::begin(&mut self.fee_rate),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"paid" => Deserialize::begin(&mut self.paid),
"platform_fee" => Deserialize::begin(&mut self.platform_fee),
"platform_fee_rate" => Deserialize::begin(&mut self.platform_fee_rate),
"refund_reason" => Deserialize::begin(&mut self.refund_reason),
"refunded" => Deserialize::begin(&mut self.refunded),
"subscription" => Deserialize::begin(&mut self.subscription),
"tenant" => Deserialize::begin(&mut self.tenant),
"term_id" => Deserialize::begin(&mut self.term_id),
"three_d_secure_status" => Deserialize::begin(&mut self.three_d_secure_status),
"total_platform_fee" => Deserialize::begin(&mut self.total_platform_fee),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            amount: Deserialize::default(),
amount_refunded: Deserialize::default(),
captured: Deserialize::default(),
captured_at: Deserialize::default(),
card: Deserialize::default(),
created: Deserialize::default(),
currency: Deserialize::default(),
customer: Deserialize::default(),
description: Deserialize::default(),
expired_at: Deserialize::default(),
failure_code: Deserialize::default(),
failure_message: Deserialize::default(),
fee_rate: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
paid: Deserialize::default(),
platform_fee: Deserialize::default(),
platform_fee_rate: Deserialize::default(),
refund_reason: Deserialize::default(),
refunded: Deserialize::default(),
subscription: Deserialize::default(),
tenant: Deserialize::default(),
term_id: Deserialize::default(),
three_d_secure_status: Deserialize::default(),
total_platform_fee: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(amount),
Some(amount_refunded),
Some(captured),
Some(captured_at),
Some(card),
Some(created),
Some(currency),
Some(customer),
Some(description),
Some(expired_at),
Some(failure_code),
Some(failure_message),
Some(fee_rate),
Some(id),
Some(livemode),
Some(metadata),
Some(paid),
Some(platform_fee),
Some(platform_fee_rate),
Some(refund_reason),
Some(refunded),
Some(subscription),
Some(tenant),
Some(term_id),
Some(three_d_secure_status),
Some(total_platform_fee),
) = (self.amount,
self.amount_refunded,
self.captured,
self.captured_at,
self.card.take(),
self.created,
self.currency,
self.customer.take(),
self.description.take(),
self.expired_at,
self.failure_code.take(),
self.failure_message.take(),
self.fee_rate.take(),
self.id.take(),
self.livemode,
self.metadata.take(),
self.paid,
self.platform_fee,
self.platform_fee_rate.take(),
self.refund_reason.take(),
self.refunded,
self.subscription.take(),
self.tenant.take(),
self.term_id.take(),
self.three_d_secure_status,
self.total_platform_fee,
) else {
            return None;
        };
        Some(Self::Out { amount,amount_refunded,captured,captured_at,card,created,currency,customer,description,expired_at,failure_code,failure_message,fee_rate,id,livemode,metadata,paid,platform_fee,platform_fee_rate,refund_reason,refunded,subscription,tenant,term_id,three_d_secure_status,total_platform_fee })
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

impl ObjectDeser for Charge {
    type Builder = ChargeBuilder;
}

impl FromValueOpt for Charge {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ChargeBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "amount" => b.amount = FromValueOpt::from_value(v),
"amount_refunded" => b.amount_refunded = FromValueOpt::from_value(v),
"captured" => b.captured = FromValueOpt::from_value(v),
"captured_at" => b.captured_at = FromValueOpt::from_value(v),
"card" => b.card = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"currency" => b.currency = FromValueOpt::from_value(v),
"customer" => b.customer = FromValueOpt::from_value(v),
"description" => b.description = FromValueOpt::from_value(v),
"expired_at" => b.expired_at = FromValueOpt::from_value(v),
"failure_code" => b.failure_code = FromValueOpt::from_value(v),
"failure_message" => b.failure_message = FromValueOpt::from_value(v),
"fee_rate" => b.fee_rate = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"paid" => b.paid = FromValueOpt::from_value(v),
"platform_fee" => b.platform_fee = FromValueOpt::from_value(v),
"platform_fee_rate" => b.platform_fee_rate = FromValueOpt::from_value(v),
"refund_reason" => b.refund_reason = FromValueOpt::from_value(v),
"refunded" => b.refunded = FromValueOpt::from_value(v),
"subscription" => b.subscription = FromValueOpt::from_value(v),
"tenant" => b.tenant = FromValueOpt::from_value(v),
"term_id" => b.term_id = FromValueOpt::from_value(v),
"three_d_secure_status" => b.three_d_secure_status = FromValueOpt::from_value(v),
"total_platform_fee" => b.total_platform_fee = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Charge {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Charge", 27)?;
        s.serialize_field("amount", &self.amount)?;
s.serialize_field("amount_refunded", &self.amount_refunded)?;
s.serialize_field("captured", &self.captured)?;
s.serialize_field("captured_at", &self.captured_at)?;
s.serialize_field("card", &self.card)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("currency", &self.currency)?;
s.serialize_field("customer", &self.customer)?;
s.serialize_field("description", &self.description)?;
s.serialize_field("expired_at", &self.expired_at)?;
s.serialize_field("failure_code", &self.failure_code)?;
s.serialize_field("failure_message", &self.failure_message)?;
s.serialize_field("fee_rate", &self.fee_rate)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("metadata", &self.metadata)?;
s.serialize_field("paid", &self.paid)?;
s.serialize_field("platform_fee", &self.platform_fee)?;
s.serialize_field("platform_fee_rate", &self.platform_fee_rate)?;
s.serialize_field("refund_reason", &self.refund_reason)?;
s.serialize_field("refunded", &self.refunded)?;
s.serialize_field("subscription", &self.subscription)?;
s.serialize_field("tenant", &self.tenant)?;
s.serialize_field("term_id", &self.term_id)?;
s.serialize_field("three_d_secure_status", &self.three_d_secure_status)?;
s.serialize_field("total_platform_fee", &self.total_platform_fee)?;

        s.serialize_field("object", "charge")?;
        s.end()
    }
}
/// 3文字のISOコード(現状 "jpy" のみサポート)
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ChargeCurrency {
Jpy,

}
impl ChargeCurrency {
    pub fn as_str(self) -> &'static str {
        use ChargeCurrency::*;
        match self {
Jpy => "jpy",

        }
    }
}

impl std::str::FromStr for ChargeCurrency {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChargeCurrency::*;
        match s {
    "jpy" => Ok(Jpy),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ChargeCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ChargeCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ChargeCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ChargeCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ChargeCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ChargeCurrency::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ChargeCurrency);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ChargeCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ChargeCurrency"))
    }
}
/// 3Dセキュアの実施状況
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ChargeThreeDSecureStatus {
Unverified,
Verified,
Attempted,
Failed,
Error,

}
impl ChargeThreeDSecureStatus {
    pub fn as_str(self) -> &'static str {
        use ChargeThreeDSecureStatus::*;
        match self {
Unverified => "unverified",
Verified => "verified",
Attempted => "attempted",
Failed => "failed",
Error => "error",

        }
    }
}

impl std::str::FromStr for ChargeThreeDSecureStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChargeThreeDSecureStatus::*;
        match s {
    "unverified" => Ok(Unverified),
"verified" => Ok(Verified),
"attempted" => Ok(Attempted),
"failed" => Ok(Failed),
"error" => Ok(Error),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ChargeThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ChargeThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ChargeThreeDSecureStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ChargeThreeDSecureStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ChargeThreeDSecureStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ChargeThreeDSecureStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ChargeThreeDSecureStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ChargeThreeDSecureStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ChargeThreeDSecureStatus"))
    }
}
impl payjp_types::Object for Charge {
    type Id = payjp_core::ChargeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(ChargeId);
