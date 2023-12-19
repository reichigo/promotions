use serde::{Deserialize, Serialize};
use crate::db::datasource::mongo::data_model::rule_items_model_mongo::RuleItemModelMongo;

#[derive(Debug, Serialize, Deserialize)]
pub struct  RuleDataModelMongo{
    pub is_for_new_user : Option<bool>,
    pub min_money : Option<f64>,
    pub min_items: Option<u32>,
    pub rule_items: Option<RuleItemModelMongo>,
}