/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Token {
pub card: payjp_core::Card,
    /// このトークン作成時のタイムスタンプ
pub created: i64,
    /// tok_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::TokenId,
    /// 本番環境かどうか
pub livemode: bool,
    /// このトークンが使用済みかどうか
pub used: bool,

}
#[doc(hidden)]
pub struct TokenBuilder {
    card: Option<payjp_core::Card>,
created: Option<i64>,
id: Option<payjp_core::TokenId>,
livemode: Option<bool>,
used: Option<bool>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Token {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Token>,
    builder: TokenBuilder,
}

impl Visitor for Place<Token> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TokenBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for TokenBuilder {
    type Out = Token;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "card" => Deserialize::begin(&mut self.card),
"created" => Deserialize::begin(&mut self.created),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"used" => Deserialize::begin(&mut self.used),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            card: Deserialize::default(),
created: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
used: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(card),
Some(created),
Some(id),
Some(livemode),
Some(used),
) = (self.card.take(),
self.created,
self.id.take(),
self.livemode,
self.used,
) else {
            return None;
        };
        Some(Self::Out { card,created,id,livemode,used })
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

impl ObjectDeser for Token {
    type Builder = TokenBuilder;
}

impl FromValueOpt for Token {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = TokenBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "card" => b.card = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"used" => b.used = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Token {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Token", 6)?;
        s.serialize_field("card", &self.card)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("used", &self.used)?;

        s.serialize_field("object", "token")?;
        s.end()
    }
}
impl payjp_types::Object for Token {
    type Id = payjp_core::TokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(TokenId);
