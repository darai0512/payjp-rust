/// 取引明細ダウンロードURLオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct StatementUrl {
    /// 有効期限のタイムスタンプ。
    /// 有効期限は発行から1時間です。
pub expires: i64,
    /// 取引明細書ダウンロードURL
pub url: String,

}
#[doc(hidden)]
pub struct StatementUrlBuilder {
    expires: Option<i64>,
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

    impl Deserialize for StatementUrl {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<StatementUrl>,
    builder: StatementUrlBuilder,
}

impl Visitor for Place<StatementUrl> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: StatementUrlBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for StatementUrlBuilder {
    type Out = StatementUrl;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "expires" => Deserialize::begin(&mut self.expires),
"url" => Deserialize::begin(&mut self.url),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            expires: Deserialize::default(),
url: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(expires),
Some(url),
) = (self.expires,
self.url.take(),
) else {
            return None;
        };
        Some(Self::Out { expires,url })
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

impl ObjectDeser for StatementUrl {
    type Builder = StatementUrlBuilder;
}

impl FromValueOpt for StatementUrl {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = StatementUrlBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "expires" => b.expires = FromValueOpt::from_value(v),
"url" => b.url = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for StatementUrl {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("StatementUrl", 3)?;
        s.serialize_field("expires", &self.expires)?;
s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "statement_url")?;
        s.end()
    }
}
