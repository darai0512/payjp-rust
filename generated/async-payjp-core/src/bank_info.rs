/// 入金先口座情報
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankInfo {
    /// 口座名義
pub bank_account_holder_name: String,
    /// 口座番号
pub bank_account_number: String,
    /// 最新振込結果
pub bank_account_status: BankInfoBankAccountStatus,
    /// 口座種別
pub bank_account_type: String,
    /// 支店番号
pub bank_branch_code: String,
    /// 銀行コード
pub bank_code: String,

}
#[doc(hidden)]
pub struct BankInfoBuilder {
    bank_account_holder_name: Option<String>,
bank_account_number: Option<String>,
bank_account_status: Option<BankInfoBankAccountStatus>,
bank_account_type: Option<String>,
bank_branch_code: Option<String>,
bank_code: Option<String>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for BankInfo {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<BankInfo>,
    builder: BankInfoBuilder,
}

impl Visitor for Place<BankInfo> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BankInfoBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for BankInfoBuilder {
    type Out = BankInfo;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "bank_account_holder_name" => Deserialize::begin(&mut self.bank_account_holder_name),
"bank_account_number" => Deserialize::begin(&mut self.bank_account_number),
"bank_account_status" => Deserialize::begin(&mut self.bank_account_status),
"bank_account_type" => Deserialize::begin(&mut self.bank_account_type),
"bank_branch_code" => Deserialize::begin(&mut self.bank_branch_code),
"bank_code" => Deserialize::begin(&mut self.bank_code),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            bank_account_holder_name: Deserialize::default(),
bank_account_number: Deserialize::default(),
bank_account_status: Deserialize::default(),
bank_account_type: Deserialize::default(),
bank_branch_code: Deserialize::default(),
bank_code: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(bank_account_holder_name),
Some(bank_account_number),
Some(bank_account_status),
Some(bank_account_type),
Some(bank_branch_code),
Some(bank_code),
) = (self.bank_account_holder_name.take(),
self.bank_account_number.take(),
self.bank_account_status,
self.bank_account_type.take(),
self.bank_branch_code.take(),
self.bank_code.take(),
) else {
            return None;
        };
        Some(Self::Out { bank_account_holder_name,bank_account_number,bank_account_status,bank_account_type,bank_branch_code,bank_code })
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

impl ObjectDeser for BankInfo {
    type Builder = BankInfoBuilder;
}

impl FromValueOpt for BankInfo {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = BankInfoBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "bank_account_holder_name" => b.bank_account_holder_name = FromValueOpt::from_value(v),
"bank_account_number" => b.bank_account_number = FromValueOpt::from_value(v),
"bank_account_status" => b.bank_account_status = FromValueOpt::from_value(v),
"bank_account_type" => b.bank_account_type = FromValueOpt::from_value(v),
"bank_branch_code" => b.bank_branch_code = FromValueOpt::from_value(v),
"bank_code" => b.bank_code = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 最新振込結果
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum BankInfoBankAccountStatus {
Success,
Failed,
Pending,

}
impl BankInfoBankAccountStatus {
    pub fn as_str(self) -> &'static str {
        use BankInfoBankAccountStatus::*;
        match self {
Success => "success",
Failed => "failed",
Pending => "pending",

        }
    }
}

impl std::str::FromStr for BankInfoBankAccountStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankInfoBankAccountStatus::*;
        match s {
    "success" => Ok(Success),
"failed" => Ok(Failed),
"pending" => Ok(Pending),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for BankInfoBankAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankInfoBankAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankInfoBankAccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankInfoBankAccountStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankInfoBankAccountStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankInfoBankAccountStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(BankInfoBankAccountStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankInfoBankAccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BankInfoBankAccountStatus"))
    }
}
