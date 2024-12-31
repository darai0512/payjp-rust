/// Accountオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Account {
    /// このアカウント作成時のUTCタイムスタンプ
pub created: i64,
    /// メールアドレス
pub email: String,
    /// acct_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::AccountId,
pub merchant: payjp_core::Merchant,
    /// このアカウントに紐付くチームID
pub team_id: Option<String>,

}
#[doc(hidden)]
pub struct AccountBuilder {
    created: Option<i64>,
email: Option<String>,
id: Option<payjp_core::AccountId>,
merchant: Option<payjp_core::Merchant>,
team_id: Option<Option<String>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Account {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Account>,
    builder: AccountBuilder,
}

impl Visitor for Place<Account> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: AccountBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for AccountBuilder {
    type Out = Account;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "created" => Deserialize::begin(&mut self.created),
"email" => Deserialize::begin(&mut self.email),
"id" => Deserialize::begin(&mut self.id),
"merchant" => Deserialize::begin(&mut self.merchant),
"team_id" => Deserialize::begin(&mut self.team_id),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            created: Deserialize::default(),
email: Deserialize::default(),
id: Deserialize::default(),
merchant: Deserialize::default(),
team_id: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(created),
Some(email),
Some(id),
Some(merchant),
Some(team_id),
) = (self.created,
self.email.take(),
self.id.take(),
self.merchant.take(),
self.team_id.take(),
) else {
            return None;
        };
        Some(Self::Out { created,email,id,merchant,team_id })
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

impl ObjectDeser for Account {
    type Builder = AccountBuilder;
}

impl FromValueOpt for Account {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = AccountBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "created" => b.created = FromValueOpt::from_value(v),
"email" => b.email = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"merchant" => b.merchant = FromValueOpt::from_value(v),
"team_id" => b.team_id = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for Account {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Account", 6)?;
        s.serialize_field("created", &self.created)?;
s.serialize_field("email", &self.email)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("merchant", &self.merchant)?;
s.serialize_field("team_id", &self.team_id)?;

        s.serialize_field("object", "account")?;
        s.end()
    }
}
impl payjp_types::Object for Account {
    type Id = payjp_core::AccountId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(AccountId);
