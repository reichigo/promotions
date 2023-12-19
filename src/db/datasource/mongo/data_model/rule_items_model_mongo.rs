use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleItemModelMongo{
    pub min_total_item: u32,
    pub is_valid_min_item_just_items_inside_promotion : bool
}