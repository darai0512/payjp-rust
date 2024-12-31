use crate::deserialization_fixture::check_object;

pub fn check_fixtures(resources: &serde_json::Value) {
    check_object::<payjp_core::Account>(resources, "account");
    check_object::<payjp_core::Balance>(resources, "balance");
    check_object::<payjp_core::Card>(resources, "card");
    check_object::<payjp_core::Charge>(resources, "charge");
    check_object::<payjp_core::Customer>(resources, "customer");
    check_object::<payjp_core::DeletedCard>(resources, "deleted_card");
    check_object::<payjp_core::DeletedCustomer>(resources, "deleted_customer");
    check_object::<payjp_core::DeletedPlan>(resources, "deleted_plan");
    check_object::<payjp_core::Event>(resources, "event");
    check_object::<payjp_core::Plan>(resources, "plan");
    check_object::<payjp_core::Subscription>(resources, "subscription");
    check_object::<payjp_core::Token>(resources, "token");
}
