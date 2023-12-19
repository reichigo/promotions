use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::entities::purchase::Purchase;
use crate::domain::entities::rule::{Rule};

pub struct PromoCode{
    id: Uuid,
    name: String,
    total_promo_code: Option<u64>,
    total_promo_code_available: Option<u64>,
    expiration_date: Option<DateTime<Utc>>,
    quantity_per_customer : u32,
    percentage_discount: Option<u32>,
    total_money_discount: Option<f64>,
    date_activate_promo_code: DateTime<Utc>,
    free_shipping: bool,
    items_promotion: Option<Vec<Uuid>>,
    rule: Rule
}
impl PromoCode{
    pub fn new(name: String,
               total_promo_code: Option<u64>,
               expiration_date: Option<DateTime<Utc>>,
               quantity_per_customer : u32,
               percentage_discount: Option<u32>,
               total_money_discount: Option<f64>,
               date_activate_promo_code: DateTime<Utc>,
               free_shipping: bool,
               items_promotion: Option<Vec<Uuid>>,
                rule: Rule) -> PromoCode{
        Self{
            id: Uuid::new_v4(),
            name,
            total_promo_code,
            total_promo_code_available: total_promo_code,
            expiration_date,
            quantity_per_customer,
            percentage_discount,
            total_money_discount,
            date_activate_promo_code,
            free_shipping,
            items_promotion,
            rule
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn percentage_discount(&self) -> Option<u32>{
        self.percentage_discount
    }

    pub fn total_money_discount(&self) -> Option<f64>{
        self.total_money_discount
    }

    pub fn total_promo_code(&self) -> Option<u64> {
        self.total_promo_code
    }

    pub fn expiration_date(&self) -> Option<DateTime<Utc>> {
        self.expiration_date
    }

    pub fn quantity_per_customer(&self) -> u32 {
        self.quantity_per_customer
    }

    pub fn rule(&self) -> &Rule{
        &self.rule
    }
    pub fn date_activate_promo_code(&self) -> DateTime<Utc> {
        self.date_activate_promo_code
    }
    pub fn free_shipping(&self) -> bool {
        self.free_shipping
    }

    pub fn is_promo_code_valid(&self, purchase: &Purchase, promo_code: &PromoCode, is_new_user: bool) -> (bool, String) {
        if let (false, message) = self.is_promo_code_available() {
            return (false, message)
        }

        if let (false, message) = self.is_promo_code_not_expired() {
            return (false, message)
        }

        if let (false, message) = self.is_promo_promo_code_active() {
            return (false, message)
        }

        if let (false, message) = self.rule().is_rule_valid(purchase, promo_code, is_new_user) {
            return (false, message)
        }

        (true, String::new())
    }

    pub fn is_promo_code_available(&self) -> (bool, String){
        let Some(total_promo_code_available) =  self.total_promo_code_available else{
            return (true, String::default());
        };

        if total_promo_code_available <= 0 {
            return (false, String::from("There is no promo code available"));
        }

        (true, String::default())
    }

    pub fn is_promo_code_not_expired(&self) -> (bool, String) {
        let Some(data) = self.expiration_date else {
            return  (true, String::default());
        };

        if data < Utc::now() {
            return (false, String::from("The promotion has expired"));
        }

        (true, String::new())
    }

    pub fn is_promo_promo_code_active(&self) -> (bool, String) {

        if self.date_activate_promo_code > Utc::now() {
            return (false, String::from("The promotion has not active"));
        }

        (true, String::new())
    }
    pub fn items_promotion(&self) -> &Option<Vec<Uuid>> {
        &self.items_promotion
    }
}

