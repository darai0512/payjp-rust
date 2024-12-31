/// Term（集計区間）オブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Term {
    /// この区間内で確定された支払いの数
pub charge_count: u64,
    /// 締め処理が完了済みならTrue
pub closed: bool,
    /// この区間内で確定されたチャージバック/チャージバックキャンセルの数
pub dispute_count: u64,
        /// 区間終了時刻のタイムスタンプ。Termが表す区間はstart_at以上end_at未満の範囲となります。翌サイクルのTermの場合`null`を返します。.
pub end_at: Option<i64>,
    /// tm_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::TermId,
    /// 本番環境かどうか。
pub livemode: bool,
    /// この区間内で確定された返金の数
pub refund_count: u64,
    /// 区間開始時刻のタイムスタンプ
pub start_at: i64,

}
#[doc(hidden)]
pub struct TermBuilder {
    charge_count: Option<u64>,
closed: Option<bool>,
dispute_count: Option<u64>,
end_at: Option<Option<i64>>,
id: Option<payjp_core::TermId>,
livemode: Option<bool>,
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

    impl Deserialize for Term {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Term>,
    builder: TermBuilder,
}

impl Visitor for Place<Term> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TermBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for TermBuilder {
    type Out = Term;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "charge_count" => Deserialize::begin(&mut self.charge_count),
"closed" => Deserialize::begin(&mut self.closed),
"dispute_count" => Deserialize::begin(&mut self.dispute_count),
"end_at" => Deserialize::begin(&mut self.end_at),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
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
Some(refund_count),
Some(start_at),
) = (self.charge_count,
self.closed,
self.dispute_count,
self.end_at,
self.id.take(),
self.livemode,
self.refund_count,
self.start_at,
) else {
            return None;
        };
        Some(Self::Out { charge_count,closed,dispute_count,end_at,id,livemode,refund_count,start_at })
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

impl ObjectDeser for Term {
    type Builder = TermBuilder;
}

impl FromValueOpt for Term {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = TermBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "charge_count" => b.charge_count = FromValueOpt::from_value(v),
"closed" => b.closed = FromValueOpt::from_value(v),
"dispute_count" => b.dispute_count = FromValueOpt::from_value(v),
"end_at" => b.end_at = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"refund_count" => b.refund_count = FromValueOpt::from_value(v),
"start_at" => b.start_at = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Term {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Term", 9)?;
        s.serialize_field("charge_count", &self.charge_count)?;
s.serialize_field("closed", &self.closed)?;
s.serialize_field("dispute_count", &self.dispute_count)?;
s.serialize_field("end_at", &self.end_at)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("refund_count", &self.refund_count)?;
s.serialize_field("start_at", &self.start_at)?;

        s.serialize_field("object", "term")?;
        s.end()
    }
}
impl payjp_types::Object for Term {
    type Id = payjp_core::TermId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(TermId);
