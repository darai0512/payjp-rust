/// キーバリューの任意データ
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Copy,Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Metadata {

}
#[doc(hidden)]
pub struct MetadataBuilder {
    
}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Metadata {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Metadata>,
    builder: MetadataBuilder,
}

impl Visitor for Place<Metadata> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: MetadataBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for MetadataBuilder {
    type Out = Metadata;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            
        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let () = () else {
            return None;
        };
        Some(Self::Out {  })
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

impl ObjectDeser for Metadata {
    type Builder = MetadataBuilder;
}

impl FromValueOpt for Metadata {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = MetadataBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                
                _ => {}
            }
        }
        b.take_out()
    }
}

};
