//! Basic tests to ensure that the plan interval types
//! are exported properly. Mainly just needs to compile.

use payjp_core::plan::RetrievePlan;
use payjp_core::{PlanId, PlanInterval};
use payjp_types::Currency;

use super::get_client;

#[test]
fn can_create_plan() {
    let client = get_client();

    let id = PlanId::from("price_123");
    let plan = RetrievePlan::new(id).send_blocking(&client).unwrap();
    assert_eq!(plan.interval, PlanInterval::Month);
    assert_eq!(plan.amount, Some(2000));
}

