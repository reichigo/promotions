use crate::domain::entities::promo_code::PromoCode;
use crate::domain::entities::purchase::Purchase;
use crate::domain::entities::rule_item::RuleItem;

pub struct  Rule{
    is_for_new_user : Option<bool>,
    min_money : Option<f64>,
    min_items: Option<u32>,
    rule_items: Option<RuleItem>,
}

impl Rule {

    pub fn new(is_for_new_user : Option<bool>,
               min_money : Option<f64>,
               min_items: Option<u32>,
               rule_items: Option<RuleItem>) -> Self{
        Self{
            is_for_new_user,
            min_money,
            min_items,
            rule_items
        }
    }

    pub fn is_for_new_user(&self) -> Option<bool> {
        self.is_for_new_user
    }
    pub fn min_money(&self) -> Option<f64> {
        self.min_money
    }
    pub fn rule_items(&self) -> Option<&RuleItem> {
        self.rule_items.as_ref()
    }

    pub fn is_rule_valid(&self, purchase: &Purchase, promo_code: &PromoCode, new_user: bool) -> (bool, String){
        if self.rule_items.is_some() {
            let (is_rule_item_valid, message) = self.rule_items.as_ref().unwrap().is_min_total_item_valid(purchase, promo_code);

            if !is_rule_item_valid{
                return (false, message);
            }
        }

        if let (false, message) = self.is_new_user_valid(new_user){
            return (false, message);
        }

        if let (false, message) = self.is_min_total_item_valid(purchase){
            return (false, message);
        }

        if let (false, message) = self.is_min_money_valid(purchase){
            return (false, message);
        }

        (true, String::new())
    }

    pub fn is_new_user_valid(&self, is_new_user: bool) -> (bool, String) {

        let Some(is_for_new_user) = self.is_for_new_user else {
            return  (true, String::default());
        };

        if is_for_new_user &&
            !is_new_user {
            return (false, String::from("This promotion is just for new users"));
        }

        (true, String::new())
    }

    pub fn is_min_total_item_valid(&self, purchase: &Purchase) -> (bool, String) {

        let (Some(total_purchase), Some(min_money_required)) = (purchase.total_amount(), self.min_money) else {
            return (true, String::new());
        };

        if min_money_required < total_purchase {
            return (false, format!("The minimum amount required for the promotion is {min_money_required}"));
        }

        (true, String::new())
    }
    pub fn is_min_money_valid(&self, purchase: &Purchase) -> (bool, String) {

        let (Some(total_purchase), Some(min_money_required)) = (purchase.total_amount(), self.min_money) else {
            return (true, String::new());
        };

        if min_money_required < total_purchase {
            return (false, format!("The minimum amount required for the promotion is {min_money_required}"));
        }

        (true, String::new())
    }
    pub fn min_items(&self) -> Option<u32> {
        self.min_items
    }
}

