use crate::db::datasource::mongo::data_model::rule_items_model_mongo::RuleItemModelMongo;
use crate::db::datasource::mongo::data_model::RuleDataModelMongo;
use crate::domain::dto::RuleDto;
use crate::domain::entities::rule::Rule;
use crate::domain::entities::rule_item::RuleItem;

impl From<&Rule> for RuleDataModelMongo{
    fn from(rule: &Rule) -> Self {

        let rule_items = match rule.rule_items() {
            None => None,
            Some(rule_item) => Some(RuleItemModelMongo::from(rule_item))
        };

        Self{
            is_for_new_user: rule.is_for_new_user(),
            min_money: rule.min_money(),
            min_items: rule.min_items(),
            rule_items
        }
    }
}

impl From<&RuleDataModelMongo> for RuleDto{
    fn from(rule_data_model_mongo: &RuleDataModelMongo) -> Self {
       Self{
           is_for_new_user: rule_data_model_mongo.is_for_new_user,
           min_money: rule_data_model_mongo.min_money,
           min_items: rule_data_model_mongo.min_items
       }
    }
}