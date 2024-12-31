/// Statementオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Statement {
    /// このStatementが関連付けられているBalanceのID
pub balance_id: Option<String>,
    /// 作成時刻のUTCタイムスタンプ
pub created: i64,
    /// オブジェクトのユニークID
pub id: payjp_core::StatementId,
    /// statement_itemオブジェクトのリスト
pub items: Vec<payjp_core::StatementItem>,
    /// livemodeオブジェクトならtrue
pub livemode: bool,
    /// 含まれるstatement_itemの金額合計
pub net: i64,
    /// オブジェクトを所有するTenantのID
pub tenant_id: Option<String>,
pub term: Option<StatementTerm>,
    /// 取引明細のタイトル
pub title: Option<String>,
    /// 取引明細の区分
#[cfg_attr(feature = "deserialize", serde(rename = "type"))]
pub type_: payjp_core::StatementType,
    /// 更新時刻のUTCタイムスタンプ
pub updated: i64,

}
#[doc(hidden)]
pub struct StatementBuilder {
    balance_id: Option<Option<String>>,
created: Option<i64>,
id: Option<payjp_core::StatementId>,
items: Option<Vec<payjp_core::StatementItem>>,
livemode: Option<bool>,
net: Option<i64>,
tenant_id: Option<Option<String>>,
term: Option<Option<StatementTerm>>,
title: Option<Option<String>>,
type_: Option<payjp_core::StatementType>,
updated: Option<i64>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Statement {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Statement>,
    builder: StatementBuilder,
}

impl Visitor for Place<Statement> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: StatementBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for StatementBuilder {
    type Out = Statement;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "balance_id" => Deserialize::begin(&mut self.balance_id),
"created" => Deserialize::begin(&mut self.created),
"id" => Deserialize::begin(&mut self.id),
"items" => Deserialize::begin(&mut self.items),
"livemode" => Deserialize::begin(&mut self.livemode),
"net" => Deserialize::begin(&mut self.net),
"tenant_id" => Deserialize::begin(&mut self.tenant_id),
"term" => Deserialize::begin(&mut self.term),
"title" => Deserialize::begin(&mut self.title),
"type" => Deserialize::begin(&mut self.type_),
"updated" => Deserialize::begin(&mut self.updated),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            balance_id: Deserialize::default(),
created: Deserialize::default(),
id: Deserialize::default(),
items: Deserialize::default(),
livemode: Deserialize::default(),
net: Deserialize::default(),
tenant_id: Deserialize::default(),
term: Deserialize::default(),
title: Deserialize::default(),
type_: Deserialize::default(),
updated: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(balance_id),
Some(created),
Some(id),
Some(items),
Some(livemode),
Some(net),
Some(tenant_id),
Some(term),
Some(title),
Some(type_),
Some(updated),
) = (self.balance_id.take(),
self.created,
self.id.take(),
self.items.take(),
self.livemode,
self.net,
self.tenant_id.take(),
self.term.take(),
self.title.take(),
self.type_,
self.updated,
) else {
            return None;
        };
        Some(Self::Out { balance_id,created,id,items,livemode,net,tenant_id,term,title,type_,updated })
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

impl ObjectDeser for Statement {
    type Builder = StatementBuilder;
}

impl FromValueOpt for Statement {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = StatementBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "balance_id" => b.balance_id = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"items" => b.items = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"net" => b.net = FromValueOpt::from_value(v),
"tenant_id" => b.tenant_id = FromValueOpt::from_value(v),
"term" => b.term = FromValueOpt::from_value(v),
"title" => b.title = FromValueOpt::from_value(v),
"type" => b.type_ = FromValueOpt::from_value(v),
"updated" => b.updated = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Statement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Statement", 12)?;
        s.serialize_field("balance_id", &self.balance_id)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("items", &self.items)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("net", &self.net)?;
s.serialize_field("tenant_id", &self.tenant_id)?;
s.serialize_field("term", &self.term)?;
s.serialize_field("title", &self.title)?;
s.serialize_field("type", &self.type_)?;
s.serialize_field("updated", &self.updated)?;

        s.serialize_field("object", "statement")?;
        s.end()
    }
}
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct StatementTerm {
    /// この区間内で確定された支払いの数
pub charge_count: u64,
    /// 締め処理が完了済みならTrue
pub closed: bool,
    /// この区間内で確定されたチャージバック/チャージバックキャンセルの数
pub dispute_count: u64,
        /// 区間終了時刻のタイムスタンプ。Termが表す区間はstart_at以上end_at未満の範囲となります。翌サイクルのTermの場合`null`を返します。.
pub end_at: Option<i64>,
    /// tm_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::StatementId,
    /// 本番環境かどうか。
pub livemode: bool,
    /// 固定文字列
pub object: StatementTermObject,
    /// この区間内で確定された返金の数
pub refund_count: u64,
    /// 区間開始時刻のタイムスタンプ
pub start_at: i64,

}
#[doc(hidden)]
pub struct StatementTermBuilder {
    charge_count: Option<u64>,
closed: Option<bool>,
dispute_count: Option<u64>,
end_at: Option<Option<i64>>,
id: Option<payjp_core::StatementId>,
livemode: Option<bool>,
object: Option<StatementTermObject>,
refund_count: Option<u64>,
start_at: Option<i64>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for StatementTerm {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<StatementTerm>,
    builder: StatementTermBuilder,
}

impl Visitor for Place<StatementTerm> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: StatementTermBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for StatementTermBuilder {
    type Out = StatementTerm;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "charge_count" => Deserialize::begin(&mut self.charge_count),
"closed" => Deserialize::begin(&mut self.closed),
"dispute_count" => Deserialize::begin(&mut self.dispute_count),
"end_at" => Deserialize::begin(&mut self.end_at),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"object" => Deserialize::begin(&mut self.object),
"refund_count" => Deserialize::begin(&mut self.refund_count),
"start_at" => Deserialize::begin(&mut self.start_at),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            charge_count: Deserialize::default(),
closed: Deserialize::default(),
dispute_count: Deserialize::default(),
end_at: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
object: Deserialize::default(),
refund_count: Deserialize::default(),
start_at: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(charge_count),
Some(closed),
Some(dispute_count),
Some(end_at),
Some(id),
Some(livemode),
Some(object),
Some(refund_count),
Some(start_at),
) = (self.charge_count,
self.closed,
self.dispute_count,
self.end_at,
self.id.take(),
self.livemode,
self.object,
self.refund_count,
self.start_at,
) else {
            return None;
        };
        Some(Self::Out { charge_count,closed,dispute_count,end_at,id,livemode,object,refund_count,start_at })
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

impl ObjectDeser for StatementTerm {
    type Builder = StatementTermBuilder;
}

impl FromValueOpt for StatementTerm {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = StatementTermBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "charge_count" => b.charge_count = FromValueOpt::from_value(v),
"closed" => b.closed = FromValueOpt::from_value(v),
"dispute_count" => b.dispute_count = FromValueOpt::from_value(v),
"end_at" => b.end_at = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"refund_count" => b.refund_count = FromValueOpt::from_value(v),
"start_at" => b.start_at = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 固定文字列
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum StatementTermObject {
Term,

}
impl StatementTermObject {
    pub fn as_str(self) -> &'static str {
        use StatementTermObject::*;
        match self {
Term => "term",

        }
    }
}

impl std::str::FromStr for StatementTermObject {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatementTermObject::*;
        match s {
    "term" => Ok(Term),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for StatementTermObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for StatementTermObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for StatementTermObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for StatementTermObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<StatementTermObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(StatementTermObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(StatementTermObject);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for StatementTermObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for StatementTermObject"))
    }
}
impl payjp_types::Object for Statement {
    type Id = payjp_core::StatementId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(StatementId);
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum StatementType {
Sales,
ServiceFee,
Forfeit,
TransferFee,
Misc,

}
impl StatementType {
    pub fn as_str(self) -> &'static str {
        use StatementType::*;
        match self {
Sales => "sales",
ServiceFee => "service_fee",
Forfeit => "forfeit",
TransferFee => "transfer_fee",
Misc => "misc",

        }
    }
}

impl std::str::FromStr for StatementType {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatementType::*;
        match s {
    "sales" => Ok(Sales),
"service_fee" => Ok(ServiceFee),
"forfeit" => Ok(Forfeit),
"transfer_fee" => Ok(TransferFee),
"misc" => Ok(Misc),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for StatementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for StatementType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<StatementType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(StatementType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(StatementType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for StatementType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for StatementType"))
    }
}
