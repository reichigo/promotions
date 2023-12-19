use serde::{Deserialize, Serialize};
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::entities::purchase::Purchase;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleItem{
    min_total_item: u32,
    is_valid_min_item_just_items_inside_promotion : bool
}

impl RuleItem{
    pub fn new( min_total_item: u32,
               is_valid_min_item_just_items_inside_promotion : bool) -> Self{
        Self{
            min_total_item,
            is_valid_min_item_just_items_inside_promotion
        }
    }

    pub fn is_rule_item_valid(&self, purchase: &Purchase, promo_code: &PromoCode) -> (bool, String){
        self.is_min_total_item_valid(purchase, promo_code)
    }

    pub fn is_min_total_item_valid(&self, purchase: &Purchase, promo_code: &PromoCode) -> (bool, String){

        let total_items: u32;

         let (Some(promotion_items), Some(purchase_items)) = (promo_code.items_promotion(), purchase.purchase_items()) else{
            return (true, String::new());
        };

        if self.is_valid_min_item_just_items_inside_promotion{
             total_items = promotion_items.iter()
                .filter(|item| purchase_items.contains(item))
                .count() as u32;
        }
        else {
            total_items = promotion_items.iter().count() as u32;
        }

        if total_items < self.min_total_item{
            return (false, format!("The promotions must be the minimum {min_total_item} items", min_total_item = self.min_total_item));
        }

        return (true, String::new());
    }
    pub fn min_total_item(&self) -> u32 {
        self.min_total_item
    }
    pub fn is_valid_min_item_just_items_inside_promotion(&self) -> bool {
        self.is_valid_min_item_just_items_inside_promotion
    }
}