use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::dto::rule_dto::RuleDto;

pub struct PromoCodeDto{
    pub id: Uuid,
    pub name: String,
    pub total_promo_code: Option<u64>,
    pub total_promo_code_available: Option<u64>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub quantity_per_customer : u32,
    pub percentage_discount: Option<u32>,
    pub total_money_discount: Option<f64>,
    pub date_activate_promo_code: DateTime<Utc>,
    pub free_shipping: bool,
    pub rule: RuleDto
}