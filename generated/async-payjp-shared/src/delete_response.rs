/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeleteResponse {
    /// 常に`true`
pub deleted: bool,
    /// 削除したオブジェクトのID
pub id: payjp_shared::DeleteResponseId,
    /// 本番環境かどうか
pub livemode: bool,

}
#[doc(hidden)]
pub struct DeleteResponseBuilder {
    deleted: Option<bool>,
id: Option<payjp_shared::DeleteResponseId>,
livemode: Option<bool>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for DeleteResponse {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<DeleteResponse>,
    builder: DeleteResponseBuilder,
}

impl Visitor for Place<DeleteResponse> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: DeleteResponseBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for DeleteResponseBuilder {
    type Out = DeleteResponse;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "deleted" => Deserialize::begin(&mut self.deleted),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            deleted: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(deleted),
Some(id),
Some(livemode),
) = (self.deleted,
self.id.take(),
self.livemode,
) else {
            return None;
        };
        Some(Self::Out { deleted,id,livemode })
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

impl ObjectDeser for DeleteResponse {
    type Builder = DeleteResponseBuilder;
}

impl FromValueOpt for DeleteResponse {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = DeleteResponseBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "deleted" => b.deleted = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
impl payjp_types::Object for DeleteResponse {
    type Id = payjp_shared::DeleteResponseId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(DeleteResponseId);
