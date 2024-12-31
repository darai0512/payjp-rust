/// Balanceオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Balance {
pub bank_info: Option<payjp_core::BankInfo>,
    /// このBalanceの清算が終了していればtrue
pub closed: bool,
    /// 精算が終了した日時のタイムスタンプ
pub closed_date: Option<payjp_types::Timestamp>,
    /// オブジェクト作成時刻のUTCタイムスタンプ
pub created: i64,
    /// 入金予定日/請求期限日のタイムスタンプ
pub due_date: Option<payjp_types::Timestamp>,
    /// ba_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::BalanceId,
    /// 本番環境かどうか
pub livemode: bool,
    /// 関連付けられているStatementの総額
pub net: i64,
    /// Balanceの状態
pub state: payjp_core::BalanceState,
    /// 関連付けられているStatementオブジェクトのリスト
pub statements: Vec<payjp_core::Statement>,
    /// オブジェクトを所有するTenantのID
pub tenant_id: Option<String>,

}
#[doc(hidden)]
pub struct BalanceBuilder {
    bank_info: Option<Option<payjp_core::BankInfo>>,
closed: Option<bool>,
closed_date: Option<Option<payjp_types::Timestamp>>,
created: Option<i64>,
due_date: Option<Option<payjp_types::Timestamp>>,
id: Option<payjp_core::BalanceId>,
livemode: Option<bool>,
net: Option<i64>,
state: Option<payjp_core::BalanceState>,
statements: Option<Vec<payjp_core::Statement>>,
tenant_id: Option<Option<String>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Balance {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Balance>,
    builder: BalanceBuilder,
}

impl Visitor for Place<Balance> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BalanceBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for BalanceBuilder {
    type Out = Balance;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "bank_info" => Deserialize::begin(&mut self.bank_info),
"closed" => Deserialize::begin(&mut self.closed),
"closed_date" => Deserialize::begin(&mut self.closed_date),
"created" => Deserialize::begin(&mut self.created),
"due_date" => Deserialize::begin(&mut self.due_date),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"net" => Deserialize::begin(&mut self.net),
"state" => Deserialize::begin(&mut self.state),
"statements" => Deserialize::begin(&mut self.statements),
"tenant_id" => Deserialize::begin(&mut self.tenant_id),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            bank_info: Deserialize::default(),
closed: Deserialize::default(),
closed_date: Deserialize::default(),
created: Deserialize::default(),
due_date: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
net: Deserialize::default(),
state: Deserialize::default(),
statements: Deserialize::default(),
tenant_id: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(bank_info),
Some(closed),
Some(closed_date),
Some(created),
Some(due_date),
Some(id),
Some(livemode),
Some(net),
Some(state),
Some(statements),
Some(tenant_id),
) = (self.bank_info.take(),
self.closed,
self.closed_date,
self.created,
self.due_date,
self.id.take(),
self.livemode,
self.net,
self.state,
self.statements.take(),
self.tenant_id.take(),
) else {
            return None;
        };
        Some(Self::Out { bank_info,closed,closed_date,created,due_date,id,livemode,net,state,statements,tenant_id })
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

impl ObjectDeser for Balance {
    type Builder = BalanceBuilder;
}

impl FromValueOpt for Balance {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = BalanceBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "bank_info" => b.bank_info = FromValueOpt::from_value(v),
"closed" => b.closed = FromValueOpt::from_value(v),
"closed_date" => b.closed_date = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"due_date" => b.due_date = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"net" => b.net = FromValueOpt::from_value(v),
"state" => b.state = FromValueOpt::from_value(v),
"statements" => b.statements = FromValueOpt::from_value(v),
"tenant_id" => b.tenant_id = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Balance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Balance", 12)?;
        s.serialize_field("bank_info", &self.bank_info)?;
s.serialize_field("closed", &self.closed)?;
s.serialize_field("closed_date", &self.closed_date)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("due_date", &self.due_date)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("net", &self.net)?;
s.serialize_field("state", &self.state)?;
s.serialize_field("statements", &self.statements)?;
s.serialize_field("tenant_id", &self.tenant_id)?;

        s.serialize_field("object", "balance")?;
        s.end()
    }
}
impl payjp_types::Object for Balance {
    type Id = payjp_core::BalanceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(BalanceId);
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum BalanceState {
Collecting,
Transfer,
Claim,

}
impl BalanceState {
    pub fn as_str(self) -> &'static str {
        use BalanceState::*;
        match self {
Collecting => "collecting",
Transfer => "transfer",
Claim => "claim",

        }
    }
}

impl std::str::FromStr for BalanceState {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BalanceState::*;
        match s {
    "collecting" => Ok(Collecting),
"transfer" => Ok(Transfer),
"claim" => Ok(Claim),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for BalanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BalanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BalanceState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BalanceState {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BalanceState> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BalanceState::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(BalanceState);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BalanceState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BalanceState"))
    }
}
