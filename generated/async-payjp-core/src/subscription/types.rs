/// subscriptionオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Subscription {
    /// 定期課金がキャンセル状態になった時のタイムスタンプ
pub canceled_at: Option<i64>,
    /// 定期課金作成時のタイムスタンプ
pub created: i64,
    /// 現在の購読期間終了時のタイムスタンプ
pub current_period_end: i64,
    /// 現在の購読期間開始時のタイムスタンプ
pub current_period_start: i64,
    /// 定期課金を購読している顧客ID
pub customer: String,
    /// `sub_`で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::SubscriptionId,
    /// 本番環境かどうか
pub livemode: bool,
    /// キーバリューの任意データ
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(with = "payjp_types::with_serde_json_opt"))]
pub metadata: Option<miniserde::json::Value>,
pub next_cycle_plan: Option<SubscriptionNextCyclePlan>,
    /// "subscription"の固定文字列
pub object: String,
    /// 定期課金が停止状態になった時のタイムスタンプ
pub paused_at: Option<i64>,
pub plan: payjp_core::Plan,
    /// 日割り課金が有効かどうか
pub prorate: bool,
        /// 停止またはキャンセル状態の定期課金が有効状態になった時のタイムスタンプ.
pub resumed_at: Option<i64>,
    /// 定期課金開始時のタイムスタンプ
pub start: i64,
    /// `trial`, `active`, `canceled` または `paused` のいずれかの値。
pub status: SubscriptionStatus,
    /// トライアル期間終了時のタイムスタンプ
pub trial_end: Option<i64>,
    /// トライアル期間開始時のタイムスタンプ
pub trial_start: Option<i64>,

}
#[doc(hidden)]
pub struct SubscriptionBuilder {
    canceled_at: Option<Option<i64>>,
created: Option<i64>,
current_period_end: Option<i64>,
current_period_start: Option<i64>,
customer: Option<String>,
id: Option<payjp_core::SubscriptionId>,
livemode: Option<bool>,
metadata: Option<Option<miniserde::json::Value>>,
next_cycle_plan: Option<Option<SubscriptionNextCyclePlan>>,
object: Option<String>,
paused_at: Option<Option<i64>>,
plan: Option<payjp_core::Plan>,
prorate: Option<bool>,
resumed_at: Option<Option<i64>>,
start: Option<i64>,
status: Option<SubscriptionStatus>,
trial_end: Option<Option<i64>>,
trial_start: Option<Option<i64>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Subscription {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Subscription>,
    builder: SubscriptionBuilder,
}

impl Visitor for Place<Subscription> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: SubscriptionBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for SubscriptionBuilder {
    type Out = Subscription;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "canceled_at" => Deserialize::begin(&mut self.canceled_at),
"created" => Deserialize::begin(&mut self.created),
"current_period_end" => Deserialize::begin(&mut self.current_period_end),
"current_period_start" => Deserialize::begin(&mut self.current_period_start),
"customer" => Deserialize::begin(&mut self.customer),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"next_cycle_plan" => Deserialize::begin(&mut self.next_cycle_plan),
"object" => Deserialize::begin(&mut self.object),
"paused_at" => Deserialize::begin(&mut self.paused_at),
"plan" => Deserialize::begin(&mut self.plan),
"prorate" => Deserialize::begin(&mut self.prorate),
"resumed_at" => Deserialize::begin(&mut self.resumed_at),
"start" => Deserialize::begin(&mut self.start),
"status" => Deserialize::begin(&mut self.status),
"trial_end" => Deserialize::begin(&mut self.trial_end),
"trial_start" => Deserialize::begin(&mut self.trial_start),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            canceled_at: Deserialize::default(),
created: Deserialize::default(),
current_period_end: Deserialize::default(),
current_period_start: Deserialize::default(),
customer: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
next_cycle_plan: Deserialize::default(),
object: Deserialize::default(),
paused_at: Deserialize::default(),
plan: Deserialize::default(),
prorate: Deserialize::default(),
resumed_at: Deserialize::default(),
start: Deserialize::default(),
status: Deserialize::default(),
trial_end: Deserialize::default(),
trial_start: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(canceled_at),
Some(created),
Some(current_period_end),
Some(current_period_start),
Some(customer),
Some(id),
Some(livemode),
Some(metadata),
Some(next_cycle_plan),
Some(object),
Some(paused_at),
Some(plan),
Some(prorate),
Some(resumed_at),
Some(start),
Some(status),
Some(trial_end),
Some(trial_start),
) = (self.canceled_at,
self.created,
self.current_period_end,
self.current_period_start,
self.customer.take(),
self.id.take(),
self.livemode,
self.metadata.take(),
self.next_cycle_plan.take(),
self.object.take(),
self.paused_at,
self.plan.take(),
self.prorate,
self.resumed_at,
self.start,
self.status,
self.trial_end,
self.trial_start,
) else {
            return None;
        };
        Some(Self::Out { canceled_at,created,current_period_end,current_period_start,customer,id,livemode,metadata,next_cycle_plan,object,paused_at,plan,prorate,resumed_at,start,status,trial_end,trial_start })
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

impl ObjectDeser for Subscription {
    type Builder = SubscriptionBuilder;
}

impl FromValueOpt for Subscription {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = SubscriptionBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"current_period_end" => b.current_period_end = FromValueOpt::from_value(v),
"current_period_start" => b.current_period_start = FromValueOpt::from_value(v),
"customer" => b.customer = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"next_cycle_plan" => b.next_cycle_plan = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"paused_at" => b.paused_at = FromValueOpt::from_value(v),
"plan" => b.plan = FromValueOpt::from_value(v),
"prorate" => b.prorate = FromValueOpt::from_value(v),
"resumed_at" => b.resumed_at = FromValueOpt::from_value(v),
"start" => b.start = FromValueOpt::from_value(v),
"status" => b.status = FromValueOpt::from_value(v),
"trial_end" => b.trial_end = FromValueOpt::from_value(v),
"trial_start" => b.trial_start = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionNextCyclePlan {
    /// プランの料金
pub amount: i64,
    /// 課金日。1から31の整数で指定可能。nullの場合は課金日を指定しない。
pub billing_day: Option<i64>,
    /// プラン作成時のタイムスタンプ
pub created: i64,
    /// 3文字のISOコード(現状 "jpy" のみサポート)
pub currency: String,
    /// `pln_`で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::SubscriptionId,
    /// 課金の間隔。`day`, `week`, `month`, `year` のいずれか。
pub interval: SubscriptionNextCyclePlanInterval,
    /// 本番環境かどうか
pub livemode: bool,
    /// キーバリューの任意データ
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(with = "payjp_types::with_serde_json_opt"))]
pub metadata: Option<miniserde::json::Value>,
    /// プラン名
pub name: String,
    /// "plan"の固定文字列
pub object: String,
    /// トライアル期間の日数
pub trial_days: u32,

}
#[doc(hidden)]
pub struct SubscriptionNextCyclePlanBuilder {
    amount: Option<i64>,
billing_day: Option<Option<i64>>,
created: Option<i64>,
currency: Option<String>,
id: Option<payjp_core::SubscriptionId>,
interval: Option<SubscriptionNextCyclePlanInterval>,
livemode: Option<bool>,
metadata: Option<Option<miniserde::json::Value>>,
name: Option<String>,
object: Option<String>,
trial_days: Option<u32>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for SubscriptionNextCyclePlan {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<SubscriptionNextCyclePlan>,
    builder: SubscriptionNextCyclePlanBuilder,
}

impl Visitor for Place<SubscriptionNextCyclePlan> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: SubscriptionNextCyclePlanBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for SubscriptionNextCyclePlanBuilder {
    type Out = SubscriptionNextCyclePlan;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "amount" => Deserialize::begin(&mut self.amount),
"billing_day" => Deserialize::begin(&mut self.billing_day),
"created" => Deserialize::begin(&mut self.created),
"currency" => Deserialize::begin(&mut self.currency),
"id" => Deserialize::begin(&mut self.id),
"interval" => Deserialize::begin(&mut self.interval),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"name" => Deserialize::begin(&mut self.name),
"object" => Deserialize::begin(&mut self.object),
"trial_days" => Deserialize::begin(&mut self.trial_days),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            amount: Deserialize::default(),
billing_day: Deserialize::default(),
created: Deserialize::default(),
currency: Deserialize::default(),
id: Deserialize::default(),
interval: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
name: Deserialize::default(),
object: Deserialize::default(),
trial_days: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(amount),
Some(billing_day),
Some(created),
Some(currency),
Some(id),
Some(interval),
Some(livemode),
Some(metadata),
Some(name),
Some(object),
Some(trial_days),
) = (self.amount,
self.billing_day,
self.created,
self.currency.take(),
self.id.take(),
self.interval,
self.livemode,
self.metadata.take(),
self.name.take(),
self.object.take(),
self.trial_days,
) else {
            return None;
        };
        Some(Self::Out { amount,billing_day,created,currency,id,interval,livemode,metadata,name,object,trial_days })
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

impl ObjectDeser for SubscriptionNextCyclePlan {
    type Builder = SubscriptionNextCyclePlanBuilder;
}

impl FromValueOpt for SubscriptionNextCyclePlan {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = SubscriptionNextCyclePlanBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "amount" => b.amount = FromValueOpt::from_value(v),
"billing_day" => b.billing_day = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"currency" => b.currency = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"interval" => b.interval = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"name" => b.name = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"trial_days" => b.trial_days = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 課金の間隔。`day`, `week`, `month`, `year` のいずれか。
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum SubscriptionNextCyclePlanInterval {
Day,
Week,
Month,
Year,

}
impl SubscriptionNextCyclePlanInterval {
    pub fn as_str(self) -> &'static str {
        use SubscriptionNextCyclePlanInterval::*;
        match self {
Day => "day",
Week => "week",
Month => "month",
Year => "year",

        }
    }
}

impl std::str::FromStr for SubscriptionNextCyclePlanInterval {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionNextCyclePlanInterval::*;
        match s {
    "day" => Ok(Day),
"week" => Ok(Week),
"month" => Ok(Month),
"year" => Ok(Year),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for SubscriptionNextCyclePlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionNextCyclePlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionNextCyclePlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionNextCyclePlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionNextCyclePlanInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionNextCyclePlanInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(SubscriptionNextCyclePlanInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionNextCyclePlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionNextCyclePlanInterval"))
    }
}
/// `trial`, `active`, `canceled` または `paused` のいずれかの値。
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum SubscriptionStatus {
Trial,
Active,
Canceled,
Paused,

}
impl SubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        use SubscriptionStatus::*;
        match self {
Trial => "trial",
Active => "active",
Canceled => "canceled",
Paused => "paused",

        }
    }
}

impl std::str::FromStr for SubscriptionStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionStatus::*;
        match s {
    "trial" => Ok(Trial),
"active" => Ok(Active),
"canceled" => Ok(Canceled),
"paused" => Ok(Paused),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(SubscriptionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionStatus"))
    }
}
impl payjp_types::Object for Subscription {
    type Id = payjp_core::SubscriptionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(SubscriptionId);
