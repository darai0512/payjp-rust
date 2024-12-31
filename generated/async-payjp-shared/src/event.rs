/// eventオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Event {
    /// このイベント作成時のUTCタイムスタンプ
     pub created: i64,
    /// このイベントに関連したリソースオブジェクト todo generics????
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "payjp_types::with_serde_json")
    )]
    pub data: miniserde::json::Value,
    /// evnt_で始まる一意なオブジェクトを示す文字列
    pub id: payjp_shared::EventId,
    /// 本番環境かどうか
    pub livemode: bool,
        /// 設定されたURLへの通知が完了していない(2xxのレスポンスが得られていない)webhookの数.
    pub pending_webhooks: Option<i64>,
        /// このイベントのタイプ。値の種類については[こちら](https://pay.jp/docs/webhook#%E3%82%A4%E3%83%99%E3%83%B3%E3%83%88).
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: EventType,
}
#[doc(hidden)]
pub struct EventBuilder {
    created: Option<i64>,
data: Option<miniserde::json::Value>,
id: Option<payjp_shared::EventId>,
livemode: Option<bool>,
pending_webhooks: Option<Option<i64>>,
type_: Option<EventType>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Event {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Event>,
    builder: EventBuilder,
}

impl Visitor for Place<Event> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: EventBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for EventBuilder {
    type Out = Event;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "created" => Deserialize::begin(&mut self.created),
"data" => Deserialize::begin(&mut self.data),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"pending_webhooks" => Deserialize::begin(&mut self.pending_webhooks),
"type" => Deserialize::begin(&mut self.type_),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            created: Deserialize::default(),
data: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
pending_webhooks: Deserialize::default(),
type_: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(created),
Some(data),
Some(id),
Some(livemode),
Some(pending_webhooks),
Some(type_),
) = (self.created,
self.data.take(),
self.id.take(),
self.livemode,
self.pending_webhooks,
self.type_,
) else {
            return None;
        };
        Some(Self::Out { created,data,id,livemode,pending_webhooks,type_ })
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

impl ObjectDeser for Event {
    type Builder = EventBuilder;
}

impl FromValueOpt for Event {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = EventBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "created" => b.created = FromValueOpt::from_value(v),
"data" => b.data = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"pending_webhooks" => b.pending_webhooks = FromValueOpt::from_value(v),
"type" => b.type_ = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/* todo
#[cfg(feature = "serialize")]
impl serde::Serialize for Event {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Event", 7)?;
        s.serialize_field("created", &self.created)?;
s.serialize_field("data", &self.data)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("pending_webhooks", &self.pending_webhooks)?;
s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "event")?;
        s.end()
    }
}
*/
/// このイベントのタイプ。値の種類については[こちら](https://pay.jp/docs/webhook#%E3%82%A4%E3%83%99%E3%83%B3%E3%83%88).
#[derive(Copy,Clone,Eq, PartialEq,)]#[non_exhaustive]pub enum EventType {
ChargeSucceeded,
ChargeFailed,
ChargeUpdated,
ChargeRefunded,
ChargeCaptured,
DisputeCreated,
TokenCreated,
CustomerCreated,
CustomerUpdated,
CustomerDeleted,
CustomerCardCreated,
CustomerCardUpdated,
CustomerCardDeleted,
PlanCreated,
PlanUpdated,
PlanDeleted,
SubscriptionCreated,
SubscriptionUpdated,
SubscriptionDeleted,
SubscriptionPaused,
SubscriptionResumed,
SubscriptionCanceled,
SubscriptionRenewed,
TransferSucceeded,
TenantCreated,
TenantDeleted,
TenantUpdated,
TenantTransferSucceeded,
TermCreated,
TermClosed,
StatementCreated,
BalanceCreated,
BalanceFixed,
BalanceClosed,
BalanceMerged,
Unknown,

}
impl EventType {
    pub fn as_str(self) -> &'static str {
        use EventType::*;
        match self {
ChargeSucceeded => "charge.succeeded",
ChargeFailed => "charge.failed",
ChargeUpdated => "charge.updated",
ChargeRefunded => "charge.refunded",
ChargeCaptured => "charge.captured",
DisputeCreated => "dispute.created",
TokenCreated => "token.created",
CustomerCreated => "customer.created",
CustomerUpdated => "customer.updated",
CustomerDeleted => "customer.deleted",
CustomerCardCreated => "customer.card.created",
CustomerCardUpdated => "customer.card.updated",
CustomerCardDeleted => "customer.card.deleted",
PlanCreated => "plan.created",
PlanUpdated => "plan.updated",
PlanDeleted => "plan.deleted",
SubscriptionCreated => "subscription.created",
SubscriptionUpdated => "subscription.updated",
SubscriptionDeleted => "subscription.deleted",
SubscriptionPaused => "subscription.paused",
SubscriptionResumed => "subscription.resumed",
SubscriptionCanceled => "subscription.canceled",
SubscriptionRenewed => "subscription.renewed",
TransferSucceeded => "transfer.succeeded",
TenantCreated => "tenant.created",
TenantDeleted => "tenant.deleted",
TenantUpdated => "tenant.updated",
TenantTransferSucceeded => "tenant_transfer.succeeded",
TermCreated => "term.created",
TermClosed => "term.closed",
StatementCreated => "statement.created",
BalanceCreated => "balance.created",
BalanceFixed => "balance.fixed",
BalanceClosed => "balance.closed",
BalanceMerged => "balance.merged",
Unknown => "unknown",

        }
    }
}

impl std::str::FromStr for EventType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EventType::*;
        match s {
    "charge.succeeded" => Ok(ChargeSucceeded),
"charge.failed" => Ok(ChargeFailed),
"charge.updated" => Ok(ChargeUpdated),
"charge.refunded" => Ok(ChargeRefunded),
"charge.captured" => Ok(ChargeCaptured),
"dispute.created" => Ok(DisputeCreated),
"token.created" => Ok(TokenCreated),
"customer.created" => Ok(CustomerCreated),
"customer.updated" => Ok(CustomerUpdated),
"customer.deleted" => Ok(CustomerDeleted),
"customer.card.created" => Ok(CustomerCardCreated),
"customer.card.updated" => Ok(CustomerCardUpdated),
"customer.card.deleted" => Ok(CustomerCardDeleted),
"plan.created" => Ok(PlanCreated),
"plan.updated" => Ok(PlanUpdated),
"plan.deleted" => Ok(PlanDeleted),
"subscription.created" => Ok(SubscriptionCreated),
"subscription.updated" => Ok(SubscriptionUpdated),
"subscription.deleted" => Ok(SubscriptionDeleted),
"subscription.paused" => Ok(SubscriptionPaused),
"subscription.resumed" => Ok(SubscriptionResumed),
"subscription.canceled" => Ok(SubscriptionCanceled),
"subscription.renewed" => Ok(SubscriptionRenewed),
"transfer.succeeded" => Ok(TransferSucceeded),
"tenant.created" => Ok(TenantCreated),
"tenant.deleted" => Ok(TenantDeleted),
"tenant.updated" => Ok(TenantUpdated),
"tenant_transfer.succeeded" => Ok(TenantTransferSucceeded),
"term.created" => Ok(TermCreated),
"term.closed" => Ok(TermClosed),
"statement.created" => Ok(StatementCreated),
"balance.created" => Ok(BalanceCreated),
"balance.fixed" => Ok(BalanceFixed),
"balance.closed" => Ok(BalanceClosed),
"balance.merged" => Ok(BalanceMerged),
_ => Ok(Self::Unknown)

        }
    }
}
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for EventType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<EventType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(EventType::from_str(s).unwrap());
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(EventType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for EventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
impl payjp_types::Object for Event {
    type Id = payjp_shared::EventId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(EventId);
