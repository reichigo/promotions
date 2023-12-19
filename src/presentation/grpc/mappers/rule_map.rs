use crate::domain::entities::rule::Rule;
use crate::domain::entities::rule_item::RuleItem;
use crate::presentation::grpc::generated::{CreatePromoCodeRuleRequest};

impl From<&CreatePromoCodeRuleRequest> for Rule{
    fn from(create_promo_code_rule_request: &CreatePromoCodeRuleRequest) -> Self {

        let create_promo_code_rule_item_request = match &create_promo_code_rule_request.create_promo_code_rule_item_request {
            None => None,
            Some(create_promo_code_rule_item_request) => Some(RuleItem::from(create_promo_code_rule_item_request))
        };

        Self::new(
            create_promo_code_rule_request.new_user,
            create_promo_code_rule_request.min_money,
            create_promo_code_rule_request.min_items,
            create_promo_code_rule_item_request
        )
    }
}