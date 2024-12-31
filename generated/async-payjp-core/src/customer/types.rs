/// Customerオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Customer {
pub cards: Option<CustomerCards>,
    /// この顧客作成時のUTCタイムスタンプ
pub created: i64,
    /// 支払いにデフォルトで使用されるカードのID
pub default_card: Option<String>,
    /// 概要
pub description: Option<String>,
    /// メールアドレス
pub email: Option<String>,
    /// 一意なオブジェクトを示す文字列
pub id: payjp_core::CustomerId,
    /// 本番環境かどうか
pub livemode: bool,
pub metadata: Option<payjp_shared::Metadata>,
pub subscriptions: Option<CustomerSubscriptions>,

}
#[doc(hidden)]
pub struct CustomerBuilder {
    cards: Option<Option<CustomerCards>>,
created: Option<i64>,
default_card: Option<Option<String>>,
description: Option<Option<String>>,
email: Option<Option<String>>,
id: Option<payjp_core::CustomerId>,
livemode: Option<bool>,
metadata: Option<Option<payjp_shared::Metadata>>,
subscriptions: Option<Option<CustomerSubscriptions>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Customer {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Customer>,
    builder: CustomerBuilder,
}

impl Visitor for Place<Customer> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for CustomerBuilder {
    type Out = Customer;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "cards" => Deserialize::begin(&mut self.cards),
"created" => Deserialize::begin(&mut self.created),
"default_card" => Deserialize::begin(&mut self.default_card),
"description" => Deserialize::begin(&mut self.description),
"email" => Deserialize::begin(&mut self.email),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"metadata" => Deserialize::begin(&mut self.metadata),
"subscriptions" => Deserialize::begin(&mut self.subscriptions),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            cards: Deserialize::default(),
created: Deserialize::default(),
default_card: Deserialize::default(),
description: Deserialize::default(),
email: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
metadata: Deserialize::default(),
subscriptions: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(cards),
Some(created),
Some(default_card),
Some(description),
Some(email),
Some(id),
Some(livemode),
Some(metadata),
Some(subscriptions),
) = (self.cards.take(),
self.created,
self.default_card.take(),
self.description.take(),
self.email.take(),
self.id.take(),
self.livemode,
self.metadata,
self.subscriptions.take(),
) else {
            return None;
        };
        Some(Self::Out { cards,created,default_card,description,email,id,livemode,metadata,subscriptions })
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

impl ObjectDeser for Customer {
    type Builder = CustomerBuilder;
}

impl FromValueOpt for Customer {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "cards" => b.cards = FromValueOpt::from_value(v),
"created" => b.created = FromValueOpt::from_value(v),
"default_card" => b.default_card = FromValueOpt::from_value(v),
"description" => b.description = FromValueOpt::from_value(v),
"email" => b.email = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"metadata" => b.metadata = FromValueOpt::from_value(v),
"subscriptions" => b.subscriptions = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Customer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Customer", 10)?;
        s.serialize_field("cards", &self.cards)?;
s.serialize_field("created", &self.created)?;
s.serialize_field("default_card", &self.default_card)?;
s.serialize_field("description", &self.description)?;
s.serialize_field("email", &self.email)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("metadata", &self.metadata)?;
s.serialize_field("subscriptions", &self.subscriptions)?;

        s.serialize_field("object", "customer")?;
        s.end()
    }
}
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerCards {
    /// 取得されたオブジェクトの総数
pub count: u64,
    /// オブジェクトの配列
pub data: Vec<payjp_core::Card>,
    /// さらにデータがあるかどうか
pub has_more: bool,
    /// 固定文字列
pub object: payjp_core::CustomerCardsObject,
    /// APIエンドポイントのURL
pub url: String,

}
#[doc(hidden)]
pub struct CustomerCardsBuilder {
    count: Option<u64>,
data: Option<Vec<payjp_core::Card>>,
has_more: Option<bool>,
object: Option<payjp_core::CustomerCardsObject>,
url: Option<String>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for CustomerCards {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<CustomerCards>,
    builder: CustomerCardsBuilder,
}

impl Visitor for Place<CustomerCards> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerCardsBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for CustomerCardsBuilder {
    type Out = CustomerCards;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "count" => Deserialize::begin(&mut self.count),
"data" => Deserialize::begin(&mut self.data),
"has_more" => Deserialize::begin(&mut self.has_more),
"object" => Deserialize::begin(&mut self.object),
"url" => Deserialize::begin(&mut self.url),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            count: Deserialize::default(),
data: Deserialize::default(),
has_more: Deserialize::default(),
object: Deserialize::default(),
url: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(count),
Some(data),
Some(has_more),
Some(object),
Some(url),
) = (self.count,
self.data.take(),
self.has_more,
self.object,
self.url.take(),
) else {
            return None;
        };
        Some(Self::Out { count,data,has_more,object,url })
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

impl ObjectDeser for CustomerCards {
    type Builder = CustomerCardsBuilder;
}

impl FromValueOpt for CustomerCards {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerCardsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "count" => b.count = FromValueOpt::from_value(v),
"data" => b.data = FromValueOpt::from_value(v),
"has_more" => b.has_more = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"url" => b.url = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSubscriptions {
    /// 取得されたオブジェクトの総数
pub count: u64,
    /// オブジェクトの配列
pub data: Vec<payjp_core::Subscription>,
    /// さらにデータがあるかどうか
pub has_more: bool,
    /// 固定文字列
pub object: payjp_core::CustomerCardsObject,
    /// APIエンドポイントのURL
pub url: String,

}
#[doc(hidden)]
pub struct CustomerSubscriptionsBuilder {
    count: Option<u64>,
data: Option<Vec<payjp_core::Subscription>>,
has_more: Option<bool>,
object: Option<payjp_core::CustomerCardsObject>,
url: Option<String>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for CustomerSubscriptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<CustomerSubscriptions>,
    builder: CustomerSubscriptionsBuilder,
}

impl Visitor for Place<CustomerSubscriptions> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSubscriptionsBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for CustomerSubscriptionsBuilder {
    type Out = CustomerSubscriptions;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "count" => Deserialize::begin(&mut self.count),
"data" => Deserialize::begin(&mut self.data),
"has_more" => Deserialize::begin(&mut self.has_more),
"object" => Deserialize::begin(&mut self.object),
"url" => Deserialize::begin(&mut self.url),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            count: Deserialize::default(),
data: Deserialize::default(),
has_more: Deserialize::default(),
object: Deserialize::default(),
url: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(count),
Some(data),
Some(has_more),
Some(object),
Some(url),
) = (self.count,
self.data.take(),
self.has_more,
self.object,
self.url.take(),
) else {
            return None;
        };
        Some(Self::Out { count,data,has_more,object,url })
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

impl ObjectDeser for CustomerSubscriptions {
    type Builder = CustomerSubscriptionsBuilder;
}

impl FromValueOpt for CustomerSubscriptions {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = CustomerSubscriptionsBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "count" => b.count = FromValueOpt::from_value(v),
"data" => b.data = FromValueOpt::from_value(v),
"has_more" => b.has_more = FromValueOpt::from_value(v),
"object" => b.object = FromValueOpt::from_value(v),
"url" => b.url = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
impl payjp_types::Object for Customer {
    type Id = payjp_core::CustomerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(CustomerId);
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum CustomerCardsObject {
List,

}
impl CustomerCardsObject {
    pub fn as_str(self) -> &'static str {
        use CustomerCardsObject::*;
        match self {
List => "list",

        }
    }
}

impl std::str::FromStr for CustomerCardsObject {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerCardsObject::*;
        match s {
    "list" => Ok(List),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for CustomerCardsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerCardsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerCardsObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerCardsObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerCardsObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerCardsObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(CustomerCardsObject);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerCardsObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerCardsObject"))
    }
}
