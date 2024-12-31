/// Statementに含まれる項目
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct StatementItem {
    /// 集計された金額
    /// 正の値は加盟店への支払額、負の値は請求額を表します。
pub amount: i64,
    /// Subjectに対応する表示名
pub name: String,
    /// 集計項目
pub subject: String,
    /// 税率(パーセント表記)。小数点以下2桁までの数値の文字列型。
pub tax_rate: String,

}
#[doc(hidden)]
pub struct StatementItemBuilder {
    amount: Option<i64>,
name: Option<String>,
subject: Option<String>,
tax_rate: Option<String>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for StatementItem {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<StatementItem>,
    builder: StatementItemBuilder,
}

impl Visitor for Place<StatementItem> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: StatementItemBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for StatementItemBuilder {
    type Out = StatementItem;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "amount" => Deserialize::begin(&mut self.amount),
"name" => Deserialize::begin(&mut self.name),
"subject" => Deserialize::begin(&mut self.subject),
"tax_rate" => Deserialize::begin(&mut self.tax_rate),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            amount: Deserialize::default(),
name: Deserialize::default(),
subject: Deserialize::default(),
tax_rate: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(amount),
Some(name),
Some(subject),
Some(tax_rate),
) = (self.amount,
self.name.take(),
self.subject.take(),
self.tax_rate.take(),
) else {
            return None;
        };
        Some(Self::Out { amount,name,subject,tax_rate })
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

impl ObjectDeser for StatementItem {
    type Builder = StatementItemBuilder;
}

impl FromValueOpt for StatementItem {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = StatementItemBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "amount" => b.amount = FromValueOpt::from_value(v),
"name" => b.name = FromValueOpt::from_value(v),
"subject" => b.subject = FromValueOpt::from_value(v),
"tax_rate" => b.tax_rate = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
