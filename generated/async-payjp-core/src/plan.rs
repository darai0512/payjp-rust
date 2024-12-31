/// planオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Plan {
    /// プランの料金
pub amount: i64,
    /// 課金日。1から31の整数で指定可能。nullの場合は課金日を指定しない。
pub billing_day: Option<i64>,
    /// プラン作成時のタイムスタンプ
pub created: i64,
    /// 3文字のISOコード(現状 "jpy" のみサポート)
pub currency: String,
    /// `pln_`で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::PlanId,
    /// 課金の間隔。`day`, `week`, `month`, `year` のいずれか。
pub interval: PlanInterval,
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
pub struct PlanBuilder {
    amount: Option<i64>,
billing_day: Option<Option<i64>>,
created: Option<i64>,
currency: Option<String>,
id: Option<payjp_core::PlanId>,
interval: Option<PlanInterval>,
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

    impl Deserialize for Plan {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Plan>,
    builder: PlanBuilder,
}

impl Visitor for Place<Plan> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PlanBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for PlanBuilder {
    type Out = Plan;
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

impl ObjectDeser for Plan {
    type Builder = PlanBuilder;
}

impl FromValueOpt for Plan {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = PlanBuilder::deser_default();
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
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum PlanInterval {
Day,
Week,
Month,
Year,

}
impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        use PlanInterval::*;
        match self {
Day => "day",
Week => "week",
Month => "month",
Year => "year",

        }
    }
}

impl std::str::FromStr for PlanInterval {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlanInterval::*;
        match s {
    "day" => Ok(Day),
"week" => Ok(Week),
"month" => Ok(Month),
"year" => Ok(Year),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PlanInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PlanInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PlanInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(PlanInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanInterval"))
    }
}
impl payjp_types::Object for Plan {
    type Id = payjp_core::PlanId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(PlanId);
