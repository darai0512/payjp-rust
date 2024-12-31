/// エラーオブジェクト
///
/// For more details see <<https://pay.jp/docs/api>>.
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Error {
pub error: Box<ErrorError>,

}
#[doc(hidden)]
pub struct ErrorBuilder {
    error: Option<Box<ErrorError>>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for Error {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<Error>,
    builder: ErrorBuilder,
}

impl Visitor for Place<Error> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ErrorBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ErrorBuilder {
    type Out = Error;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "error" => Deserialize::begin(&mut self.error),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            error: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(error),
) = (self.error.take(),
) else {
            return None;
        };
        Some(Self::Out { error })
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

impl ObjectDeser for Error {
    type Builder = ErrorBuilder;
}

impl FromValueOpt for Error {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ErrorBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "error" => b.error = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
#[derive(Clone,Debug,)]#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ErrorError {
    /// エラーに関連する支払いID
pub charge: Option<String>,
    /// 詳細なエラー内容の識別子が入ります
pub code: Option<ErrorErrorCode>,
        /// エラーメッセージ。これは事業者向けの内容となっており、エンドユーザーへ提示する内容として利用することは推奨しておりません。.
pub message: String,
    /// エラーに関連するパラメータ名
pub param: Option<String>,
    /// HTTPステータスコードと同様の値が数値型で入ります
pub status: i64,
    /// 大まかなエラー種別が入ります
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: ErrorErrorType,

}
#[doc(hidden)]
pub struct ErrorErrorBuilder {
    charge: Option<Option<String>>,
code: Option<Option<ErrorErrorCode>>,
message: Option<String>,
param: Option<Option<String>>,
status: Option<i64>,
type_: Option<ErrorErrorType>,

}

#[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use payjp_types::{MapBuilder, ObjectDeser};
    use payjp_types::miniserde_helpers::FromValueOpt;

    make_place!(Place);

    impl Deserialize for ErrorError {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

struct Builder<'a> {
    out: &'a mut Option<ErrorError>,
    builder: ErrorErrorBuilder,
}

impl Visitor for Place<ErrorError> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: ErrorErrorBuilder::deser_default(),
        }))
    }
}

impl MapBuilder for ErrorErrorBuilder {
    type Out = ErrorError;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "charge" => Deserialize::begin(&mut self.charge),
"code" => Deserialize::begin(&mut self.code),
"message" => Deserialize::begin(&mut self.message),
"param" => Deserialize::begin(&mut self.param),
"status" => Deserialize::begin(&mut self.status),
"type" => Deserialize::begin(&mut self.type_),

            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self {
            charge: Deserialize::default(),
code: Deserialize::default(),
message: Deserialize::default(),
param: Deserialize::default(),
status: Deserialize::default(),
type_: Deserialize::default(),

        }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(charge),
Some(code),
Some(message),
Some(param),
Some(status),
Some(type_),
) = (self.charge.take(),
self.code,
self.message.take(),
self.param.take(),
self.status,
self.type_,
) else {
            return None;
        };
        Some(Self::Out { charge,code,message,param,status,type_ })
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

impl ObjectDeser for ErrorError {
    type Builder = ErrorErrorBuilder;
}

impl FromValueOpt for ErrorError {
    fn from_value(v: Value) -> Option<Self> {
        let Value::Object(obj) = v else {
            return None;
        };
        let mut b = ErrorErrorBuilder::deser_default();
        for (k, v) in obj {
            match k.as_str() {
                "charge" => b.charge = FromValueOpt::from_value(v),
"code" => b.code = FromValueOpt::from_value(v),
"message" => b.message = FromValueOpt::from_value(v),
"param" => b.param = FromValueOpt::from_value(v),
"status" => b.status = FromValueOpt::from_value(v),
"type" => b.type_ = FromValueOpt::from_value(v),

                _ => {}
            }
        }
        b.take_out()
    }
}

};
/// 詳細なエラー内容の識別子が入ります
#[derive(Copy,Clone,Eq, PartialEq,)]#[non_exhaustive]pub enum ErrorErrorCode {
InvalidNumber,
InvalidCvc,
InvalidExpirationDate,
IncorrectCardData,
InvalidExpiryMonth,
InvalidExpiryYear,
ExpiredCard,
CardDeclined,
CardFlagged,
ProcessingError,
MissingCard,
UnacceptableBrand,
InvalidId,
NoApiKey,
InvalidApiKey,
InvalidPlan,
InvalidExpiryDays,
UnnecessaryExpiryDays,
InvalidFlexibleId,
InvalidTimestamp,
InvalidTrialEnd,
InvalidStringLength,
InvalidCountry,
InvalidCurrency,
InvalidAddressZip,
InvalidAmount,
InvalidPlanAmount,
InvalidCard,
InvalidCardName,
InvalidCardCountry,
InvalidCardAddressZip,
InvalidCardAddressState,
InvalidCardAddressCity,
InvalidCardAddressLine,
InvalidCustomer,
InvalidBoolean,
InvalidEmail,
NoAllowedParam,
NoParam,
InvalidQuerystring,
MissingParam,
InvalidParamKey,
NoPaymentMethod,
PaymentMethodDuplicate,
PaymentMethodDuplicateIncludingCustomer,
FailedPayment,
InvalidRefundAmount,
AlreadyRefunded,
InvalidAmountToNotCaptured,
RefundAmountGtNet,
CaptureAmountGtNet,
InvalidRefundReason,
AlreadyCaptured,
CantCaptureRefundedCharge,
CantReauthRefundedCharge,
ChargeExpired,
AlreadyExistId,
TokenAlreadyUsed,
AlreadyHaveCard,
DontHasThisCard,
DoesntHaveCard,
AlreadyHaveTheSameCard,
InvalidInterval,
InvalidTrialDays,
InvalidBillingDay,
BillingDayForNonMonthlyPlan,
ExistSubscribers,
AlreadySubscribed,
AlreadyCanceled,
AlreadyPaused,
SubscriptionWorked,
CannotChangeProrateStatus,
TooManyMetadataKeys,
InvalidMetadataKey,
InvalidMetadataValue,
ApplePayDisabledInLivemode,
InvalidApplePayToken,
TestCardOnLivemode,
NotActivatedAccount,
PayjpWrong,
PgWrong,
NotFound,
NotAllowedMethod,
OverCapacity,
RefundLimitExceeded,
CannotProratedRefundOfSubscription,
ThreeDSecureIncompleted,
ThreeDSecureFailed,
NotInThreeDSecureFlow,
UnverifiedToken,
InvalidOwnerType,
Unknown,

}
impl ErrorErrorCode {
    pub fn as_str(self) -> &'static str {
        use ErrorErrorCode::*;
        match self {
InvalidNumber => "invalid_number",
InvalidCvc => "invalid_cvc",
InvalidExpirationDate => "invalid_expiration_date",
IncorrectCardData => "incorrect_card_data",
InvalidExpiryMonth => "invalid_expiry_month",
InvalidExpiryYear => "invalid_expiry_year",
ExpiredCard => "expired_card",
CardDeclined => "card_declined",
CardFlagged => "card_flagged",
ProcessingError => "processing_error",
MissingCard => "missing_card",
UnacceptableBrand => "unacceptable_brand",
InvalidId => "invalid_id",
NoApiKey => "no_api_key",
InvalidApiKey => "invalid_api_key",
InvalidPlan => "invalid_plan",
InvalidExpiryDays => "invalid_expiry_days",
UnnecessaryExpiryDays => "unnecessary_expiry_days",
InvalidFlexibleId => "invalid_flexible_id",
InvalidTimestamp => "invalid_timestamp",
InvalidTrialEnd => "invalid_trial_end",
InvalidStringLength => "invalid_string_length",
InvalidCountry => "invalid_country",
InvalidCurrency => "invalid_currency",
InvalidAddressZip => "invalid_address_zip",
InvalidAmount => "invalid_amount",
InvalidPlanAmount => "invalid_plan_amount",
InvalidCard => "invalid_card",
InvalidCardName => "invalid_card_name",
InvalidCardCountry => "invalid_card_country",
InvalidCardAddressZip => "invalid_card_address_zip",
InvalidCardAddressState => "invalid_card_address_state",
InvalidCardAddressCity => "invalid_card_address_city",
InvalidCardAddressLine => "invalid_card_address_line",
InvalidCustomer => "invalid_customer",
InvalidBoolean => "invalid_boolean",
InvalidEmail => "invalid_email",
NoAllowedParam => "no_allowed_param",
NoParam => "no_param",
InvalidQuerystring => "invalid_querystring",
MissingParam => "missing_param",
InvalidParamKey => "invalid_param_key",
NoPaymentMethod => "no_payment_method",
PaymentMethodDuplicate => "payment_method_duplicate",
PaymentMethodDuplicateIncludingCustomer => "payment_method_duplicate_including_customer",
FailedPayment => "failed_payment",
InvalidRefundAmount => "invalid_refund_amount",
AlreadyRefunded => "already_refunded",
InvalidAmountToNotCaptured => "invalid_amount_to_not_captured",
RefundAmountGtNet => "refund_amount_gt_net",
CaptureAmountGtNet => "capture_amount_gt_net",
InvalidRefundReason => "invalid_refund_reason",
AlreadyCaptured => "already_captured",
CantCaptureRefundedCharge => "cant_capture_refunded_charge",
CantReauthRefundedCharge => "cant_reauth_refunded_charge",
ChargeExpired => "charge_expired",
AlreadyExistId => "already_exist_id",
TokenAlreadyUsed => "token_already_used",
AlreadyHaveCard => "already_have_card",
DontHasThisCard => "dont_has_this_card",
DoesntHaveCard => "doesnt_have_card",
AlreadyHaveTheSameCard => "already_have_the_same_card",
InvalidInterval => "invalid_interval",
InvalidTrialDays => "invalid_trial_days",
InvalidBillingDay => "invalid_billing_day",
BillingDayForNonMonthlyPlan => "billing_day_for_non_monthly_plan",
ExistSubscribers => "exist_subscribers",
AlreadySubscribed => "already_subscribed",
AlreadyCanceled => "already_canceled",
AlreadyPaused => "already_paused",
SubscriptionWorked => "subscription_worked",
CannotChangeProrateStatus => "cannot_change_prorate_status",
TooManyMetadataKeys => "too_many_metadata_keys",
InvalidMetadataKey => "invalid_metadata_key",
InvalidMetadataValue => "invalid_metadata_value",
ApplePayDisabledInLivemode => "apple_pay_disabled_in_livemode",
InvalidApplePayToken => "invalid_apple_pay_token",
TestCardOnLivemode => "test_card_on_livemode",
NotActivatedAccount => "not_activated_account",
PayjpWrong => "payjp_wrong",
PgWrong => "pg_wrong",
NotFound => "not_found",
NotAllowedMethod => "not_allowed_method",
OverCapacity => "over_capacity",
RefundLimitExceeded => "refund_limit_exceeded",
CannotProratedRefundOfSubscription => "cannot_prorated_refund_of_subscription",
ThreeDSecureIncompleted => "three_d_secure_incompleted",
ThreeDSecureFailed => "three_d_secure_failed",
NotInThreeDSecureFlow => "not_in_three_d_secure_flow",
UnverifiedToken => "unverified_token",
InvalidOwnerType => "invalid_owner_type",
Unknown => "unknown",

        }
    }
}

impl std::str::FromStr for ErrorErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ErrorErrorCode::*;
        match s {
    "invalid_number" => Ok(InvalidNumber),
"invalid_cvc" => Ok(InvalidCvc),
"invalid_expiration_date" => Ok(InvalidExpirationDate),
"incorrect_card_data" => Ok(IncorrectCardData),
"invalid_expiry_month" => Ok(InvalidExpiryMonth),
"invalid_expiry_year" => Ok(InvalidExpiryYear),
"expired_card" => Ok(ExpiredCard),
"card_declined" => Ok(CardDeclined),
"card_flagged" => Ok(CardFlagged),
"processing_error" => Ok(ProcessingError),
"missing_card" => Ok(MissingCard),
"unacceptable_brand" => Ok(UnacceptableBrand),
"invalid_id" => Ok(InvalidId),
"no_api_key" => Ok(NoApiKey),
"invalid_api_key" => Ok(InvalidApiKey),
"invalid_plan" => Ok(InvalidPlan),
"invalid_expiry_days" => Ok(InvalidExpiryDays),
"unnecessary_expiry_days" => Ok(UnnecessaryExpiryDays),
"invalid_flexible_id" => Ok(InvalidFlexibleId),
"invalid_timestamp" => Ok(InvalidTimestamp),
"invalid_trial_end" => Ok(InvalidTrialEnd),
"invalid_string_length" => Ok(InvalidStringLength),
"invalid_country" => Ok(InvalidCountry),
"invalid_currency" => Ok(InvalidCurrency),
"invalid_address_zip" => Ok(InvalidAddressZip),
"invalid_amount" => Ok(InvalidAmount),
"invalid_plan_amount" => Ok(InvalidPlanAmount),
"invalid_card" => Ok(InvalidCard),
"invalid_card_name" => Ok(InvalidCardName),
"invalid_card_country" => Ok(InvalidCardCountry),
"invalid_card_address_zip" => Ok(InvalidCardAddressZip),
"invalid_card_address_state" => Ok(InvalidCardAddressState),
"invalid_card_address_city" => Ok(InvalidCardAddressCity),
"invalid_card_address_line" => Ok(InvalidCardAddressLine),
"invalid_customer" => Ok(InvalidCustomer),
"invalid_boolean" => Ok(InvalidBoolean),
"invalid_email" => Ok(InvalidEmail),
"no_allowed_param" => Ok(NoAllowedParam),
"no_param" => Ok(NoParam),
"invalid_querystring" => Ok(InvalidQuerystring),
"missing_param" => Ok(MissingParam),
"invalid_param_key" => Ok(InvalidParamKey),
"no_payment_method" => Ok(NoPaymentMethod),
"payment_method_duplicate" => Ok(PaymentMethodDuplicate),
"payment_method_duplicate_including_customer" => Ok(PaymentMethodDuplicateIncludingCustomer),
"failed_payment" => Ok(FailedPayment),
"invalid_refund_amount" => Ok(InvalidRefundAmount),
"already_refunded" => Ok(AlreadyRefunded),
"invalid_amount_to_not_captured" => Ok(InvalidAmountToNotCaptured),
"refund_amount_gt_net" => Ok(RefundAmountGtNet),
"capture_amount_gt_net" => Ok(CaptureAmountGtNet),
"invalid_refund_reason" => Ok(InvalidRefundReason),
"already_captured" => Ok(AlreadyCaptured),
"cant_capture_refunded_charge" => Ok(CantCaptureRefundedCharge),
"cant_reauth_refunded_charge" => Ok(CantReauthRefundedCharge),
"charge_expired" => Ok(ChargeExpired),
"already_exist_id" => Ok(AlreadyExistId),
"token_already_used" => Ok(TokenAlreadyUsed),
"already_have_card" => Ok(AlreadyHaveCard),
"dont_has_this_card" => Ok(DontHasThisCard),
"doesnt_have_card" => Ok(DoesntHaveCard),
"already_have_the_same_card" => Ok(AlreadyHaveTheSameCard),
"invalid_interval" => Ok(InvalidInterval),
"invalid_trial_days" => Ok(InvalidTrialDays),
"invalid_billing_day" => Ok(InvalidBillingDay),
"billing_day_for_non_monthly_plan" => Ok(BillingDayForNonMonthlyPlan),
"exist_subscribers" => Ok(ExistSubscribers),
"already_subscribed" => Ok(AlreadySubscribed),
"already_canceled" => Ok(AlreadyCanceled),
"already_paused" => Ok(AlreadyPaused),
"subscription_worked" => Ok(SubscriptionWorked),
"cannot_change_prorate_status" => Ok(CannotChangeProrateStatus),
"too_many_metadata_keys" => Ok(TooManyMetadataKeys),
"invalid_metadata_key" => Ok(InvalidMetadataKey),
"invalid_metadata_value" => Ok(InvalidMetadataValue),
"apple_pay_disabled_in_livemode" => Ok(ApplePayDisabledInLivemode),
"invalid_apple_pay_token" => Ok(InvalidApplePayToken),
"test_card_on_livemode" => Ok(TestCardOnLivemode),
"not_activated_account" => Ok(NotActivatedAccount),
"payjp_wrong" => Ok(PayjpWrong),
"pg_wrong" => Ok(PgWrong),
"not_found" => Ok(NotFound),
"not_allowed_method" => Ok(NotAllowedMethod),
"over_capacity" => Ok(OverCapacity),
"refund_limit_exceeded" => Ok(RefundLimitExceeded),
"cannot_prorated_refund_of_subscription" => Ok(CannotProratedRefundOfSubscription),
"three_d_secure_incompleted" => Ok(ThreeDSecureIncompleted),
"three_d_secure_failed" => Ok(ThreeDSecureFailed),
"not_in_three_d_secure_flow" => Ok(NotInThreeDSecureFlow),
"unverified_token" => Ok(UnverifiedToken),
"invalid_owner_type" => Ok(InvalidOwnerType),
_ => Ok(Self::Unknown)

        }
    }
}
impl std::fmt::Display for ErrorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ErrorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ErrorErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ErrorErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ErrorErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ErrorErrorCode::from_str(s).unwrap());
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ErrorErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ErrorErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// 大まかなエラー種別が入ります
#[derive(Copy,Clone,Eq, PartialEq,)]pub enum ErrorErrorType {
ClientError,
CardError,
ServerError,
NotAllowedMethodError,
AuthError,
InvalidRequestError,

}
impl ErrorErrorType {
    pub fn as_str(self) -> &'static str {
        use ErrorErrorType::*;
        match self {
ClientError => "client_error",
CardError => "card_error",
ServerError => "server_error",
NotAllowedMethodError => "not_allowed_method_error",
AuthError => "auth_error",
InvalidRequestError => "invalid_request_error",

        }
    }
}

impl std::str::FromStr for ErrorErrorType {
    type Err = payjp_types::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ErrorErrorType::*;
        match s {
    "client_error" => Ok(ClientError),
"card_error" => Ok(CardError),
"server_error" => Ok(ServerError),
"not_allowed_method_error" => Ok(NotAllowedMethodError),
"auth_error" => Ok(AuthError),
"invalid_request_error" => Ok(InvalidRequestError),
_ => Err(payjp_types::ParseError)

        }
    }
}
impl std::fmt::Display for ErrorErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ErrorErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ErrorErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ErrorErrorType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ErrorErrorType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ErrorErrorType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

payjp_types::impl_from_val_with_from_str!(ErrorErrorType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ErrorErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ErrorErrorType"))
    }
}
