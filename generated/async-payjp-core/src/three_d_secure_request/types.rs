/// three_d_secure_requestオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureRequest {
    /// 3Dセキュアリクエスト作成時のUTCタイムスタンプ
pub created: i64,
    /// 3Dセキュアリクエストが期限切れとなった時刻のUTCタイムスタンプ
    /// 3Dセキュア認証が完了していれば値はセットされません。
pub expired_at: Option<i64>,
    /// 認証終了、かつ3Dセキュアリクエストが完了した時のUTCタイムスタンプ
        /// 顧客カードに対する3Dセキュアにおいてはカード会社画面での認証を終えた後にセットされます。.
pub finished_at: Option<i64>,
    /// tdsr_で始まる一意なオブジェクトを示す文字列
pub id: payjp_core::ThreeDSecureRequestId,
    /// 本番環境かどうか
pub livemode: bool,
    /// 3Dセキュア処理対象リソースのID
pub resource_id: String,
    /// 認証終了時のUTCタイムスタンプ
    /// カード会社画面での認証を終えた後にセットされます。
pub result_received_at: Option<i64>,
    /// 認証開始時のUTCタイムスタンプ
    /// カード会社画面での認証を始める際にセットされます。
pub started_at: Option<i64>,
    /// 3Dセキュア認証の現在の状態
pub state: ThreeDSecureRequestState,
    /// テナントID
pub tenant_id: Option<String>,
    /// 3Dセキュアの認証結果
    /// 値については`charge.three_d_secure_status`に同じ
pub three_d_secure_status: Option<ThreeDSecureRequestThreeDSecureStatus>,

}
#[doc(hidden)]
pub struct ThreeDSecureRequestBuilder {
    created: Option<i64>,
expired_at: Option<Option<i64>>,
finished_at: Option<Option<i64>>,
id: Option<payjp_core::ThreeDSecureRequestId>,
livemode: Option<bool>,
resource_id: Option<String>,
result_received_at: Option<Option<i64>>,
started_at: Option<Option<i64>>,
state: Option<ThreeDSecureRequestState>,
tenant_id: Option<Option<String>>,
three_d_secure_status: Option<Option<ThreeDSecureRequestThreeDSecureStatus>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for ThreeDSecureRequest {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<ThreeDSecureRequest>,
    builder: ThreeDSecureRequestBuilder,
}

impl Visitor for Place<ThreeDSecureRequest> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ThreeDSecureRequestBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ThreeDSecureRequestBuilder {
    type Out = ThreeDSecureRequest;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "created" => Deserialize::begin(&mut self.created),
"expired_at" => Deserialize::begin(&mut self.expired_at),
"finished_at" => Deserialize::begin(&mut self.finished_at),
"id" => Deserialize::begin(&mut self.id),
"livemode" => Deserialize::begin(&mut self.livemode),
"resource_id" => Deserialize::begin(&mut self.resource_id),
"result_received_at" => Deserialize::begin(&mut self.result_received_at),
"started_at" => Deserialize::begin(&mut self.started_at),
"state" => Deserialize::begin(&mut self.state),
"tenant_id" => Deserialize::begin(&mut self.tenant_id),
"three_d_secure_status" => Deserialize::begin(&mut self.three_d_secure_status),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            created: Deserialize::default(),
expired_at: Deserialize::default(),
finished_at: Deserialize::default(),
id: Deserialize::default(),
livemode: Deserialize::default(),
resource_id: Deserialize::default(),
result_received_at: Deserialize::default(),
started_at: Deserialize::default(),
state: Deserialize::default(),
tenant_id: Deserialize::default(),
three_d_secure_status: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(created),
Some(expired_at),
Some(finished_at),
Some(id),
Some(livemode),
Some(resource_id),
Some(result_received_at),
Some(started_at),
Some(state),
Some(tenant_id),
Some(three_d_secure_status),
) = (self.created,
self.expired_at,
self.finished_at,
self.id.take(),
self.livemode,
self.resource_id.take(),
self.result_received_at,
self.started_at,
self.state,
self.tenant_id.take(),
self.three_d_secure_status,
) else {
            return None;
        };
        Some(Self::Out { created,expired_at,finished_at,id,livemode,resource_id,result_received_at,started_at,state,tenant_id,three_d_secure_status })
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

impl ObjectDeser for ThreeDSecureRequest {
    type Builder = ThreeDSecureRequestBuilder;
}

impl FromValueOpt for ThreeDSecureRequest {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ThreeDSecureRequestBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "created" => b.created = FromValueOpt::from_value(v),
"expired_at" => b.expired_at = FromValueOpt::from_value(v),
"finished_at" => b.finished_at = FromValueOpt::from_value(v),
"id" => b.id = FromValueOpt::from_value(v),
"livemode" => b.livemode = FromValueOpt::from_value(v),
"resource_id" => b.resource_id = FromValueOpt::from_value(v),
"result_received_at" => b.result_received_at = FromValueOpt::from_value(v),
"started_at" => b.started_at = FromValueOpt::from_value(v),
"state" => b.state = FromValueOpt::from_value(v),
"tenant_id" => b.tenant_id = FromValueOpt::from_value(v),
"three_d_secure_status" => b.three_d_secure_status = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureRequest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ThreeDSecureRequest", 12)?;
        s.serialize_field("created", &self.created)?;
s.serialize_field("expired_at", &self.expired_at)?;
s.serialize_field("finished_at", &self.finished_at)?;
s.serialize_field("id", &self.id)?;
s.serialize_field("livemode", &self.livemode)?;
s.serialize_field("resource_id", &self.resource_id)?;
s.serialize_field("result_received_at", &self.result_received_at)?;
s.serialize_field("started_at", &self.started_at)?;
s.serialize_field("state", &self.state)?;
s.serialize_field("tenant_id", &self.tenant_id)?;
s.serialize_field("three_d_secure_status", &self.three_d_secure_status)?;

        s.serialize_field("object", "three_d_secure_request")?;
        s.end()
    }
}
/// 3Dセキュア認証の現在の状態
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ThreeDSecureRequestState {
Created,
InProgress,
ResultReceived,
Finished,

}
impl ThreeDSecureRequestState {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureRequestState::*;
        match self {
Created => "created",
InProgress => "in_progress",
ResultReceived => "result_received",
Finished => "finished",

        }
    }
}

impl std::str::FromStr for ThreeDSecureRequestState {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureRequestState::*;
        match s {
    "created" => Ok(Created),
"in_progress" => Ok(InProgress),
"result_received" => Ok(ResultReceived),
"finished" => Ok(Finished),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ThreeDSecureRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureRequestState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureRequestState {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureRequestState> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureRequestState::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ThreeDSecureRequestState);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureRequestState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureRequestState"))
    }
}
/// 3Dセキュアの認証結果
/// 値については`charge.three_d_secure_status`に同じ
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ThreeDSecureRequestThreeDSecureStatus {
Unverified,
Verified,
Attempted,
NotSupported,
Error,
Failed,
NotEnrolled,

}
impl ThreeDSecureRequestThreeDSecureStatus {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureRequestThreeDSecureStatus::*;
        match self {
Unverified => "unverified",
Verified => "verified",
Attempted => "attempted",
NotSupported => "not_supported",
Error => "error",
Failed => "failed",
NotEnrolled => "not_enrolled",

        }
    }
}

impl std::str::FromStr for ThreeDSecureRequestThreeDSecureStatus {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureRequestThreeDSecureStatus::*;
        match s {
    "unverified" => Ok(Unverified),
"verified" => Ok(Verified),
"attempted" => Ok(Attempted),
"not_supported" => Ok(NotSupported),
"error" => Ok(Error),
"failed" => Ok(Failed),
"not_enrolled" => Ok(NotEnrolled),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ThreeDSecureRequestThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureRequestThreeDSecureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureRequestThreeDSecureStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureRequestThreeDSecureStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureRequestThreeDSecureStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureRequestThreeDSecureStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ThreeDSecureRequestThreeDSecureStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureRequestThreeDSecureStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureRequestThreeDSecureStatus"))
    }
}
impl payjp_types::Object for ThreeDSecureRequest {
    type Id = payjp_core::ThreeDSecureRequestId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
payjp_types::def_id!(ThreeDSecureRequestId);
