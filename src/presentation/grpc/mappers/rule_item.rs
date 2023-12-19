use crate::domain::entities::rule_item::RuleItem;
use crate::presentation::grpc::generated::CreatePromoCodeRuleItemRequest;

impl From<&CreatePromoCodeRuleItemRequest> for RuleItem{
    fn from(rule_item: &CreatePromoCodeRuleItemRequest) -> Self {
        Self::new(rule_item.min_total_item,
        rule_item.is_valid_min_item_just_items_inside_promotion)
    }
}