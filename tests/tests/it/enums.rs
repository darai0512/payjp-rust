use std::str::FromStr;
use payjp_core::EventType;

#[test]
fn from_str_and_deser_behavior_match_on_unknown_variant() {
    assert_eq!(EventType::Unknown, EventType::from_str("acct").unwrap());
    assert_eq!(miniserde::json::from_str::<EventType>(r#""acct""#).unwrap(), EventType::Unknown);
    assert_eq!(serde_json::from_str::<EventType>(r#""acct""#).unwrap(), EventType::Unknown);
}
