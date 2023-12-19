use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::datasource::mongo::data_model::RuleDataModelMongo;

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoCodeDataModelMongo{
    pub _id: bson::Uuid,
    pub name: String,
    pub total_promo_code: Option<u64>,
    pub total_promo_code_available: Option<u64>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub quantity_per_customer : u32,
    pub percentage_discount: Option<u32>,
    pub total_money_discount: Option<f64>,
    pub date_activate_promo_code: DateTime<Utc>,
    pub free_shipping: bool,
    pub items_promotion: Option<Vec<bson::Uuid>>,
    pub rule: RuleDataModelMongo
}