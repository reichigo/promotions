use crate::db::datasource::mongo::data_model::rule_items_model_mongo::RuleItemModelMongo;
use crate::domain::entities::rule_item::RuleItem;

impl From<&RuleItem> for RuleItemModelMongo{
    fn from(rule_item: &RuleItem) -> Self {
        Self{
            min_total_item: rule_item.min_total_item(),
            is_valid_min_item_just_items_inside_promotion: rule_item.is_valid_min_item_just_items_inside_promotion()
        }
    }
}