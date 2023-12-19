use mongodb::bson;
use uuid::Uuid;
use crate::db::datasource::mongo::data_model::PromoCodeDataModelMongo;
use crate::domain::dto::PromoCodeDto;
use crate::domain::entities::promo_code::PromoCode;

impl From<&PromoCode> for PromoCodeDataModelMongo{
    fn from(promo_code: &PromoCode) -> Self {
        
        let promotion_ids = match promo_code.items_promotion() {
            None => None,
            Some(ids) => Some(
                ids.iter()
                    .map(|id|
                        bson::Uuid::parse_str(&id.to_string())
                            .unwrap())
                    .collect())
        };
        
        Self{
            _id: bson::Uuid::parse_str(promo_code.id().to_string()).unwrap(),
            name: promo_code.name().to_string(),
            expiration_date: promo_code.expiration_date(),
            total_promo_code: promo_code.total_promo_code(),
            total_promo_code_available: promo_code.total_promo_code(),
            quantity_per_customer: promo_code.quantity_per_customer(),
            total_money_discount: Some(33.3),
            percentage_discount: promo_code.percentage_discount(),
            date_activate_promo_code: promo_code.date_activate_promo_code(),
            free_shipping: promo_code.free_shipping(),
            items_promotion: promotion_ids,
            rule: promo_code.rule().into(),
        }
    }
}

impl From<&PromoCodeDataModelMongo> for PromoCodeDto{
    fn from(promo_code_data_model_mongo: &PromoCodeDataModelMongo) -> Self {
        Self{
            id: Uuid::parse_str(&promo_code_data_model_mongo._id.to_string()).unwrap(),
            name: promo_code_data_model_mongo.name.to_string(),
            total_promo_code_available: promo_code_data_model_mongo.total_promo_code_available,
            expiration_date: None,
            quantity_per_customer: promo_code_data_model_mongo.quantity_per_customer,
            total_promo_code: promo_code_data_model_mongo.total_promo_code,
            total_money_discount: promo_code_data_model_mongo.total_money_discount,
            date_activate_promo_code: promo_code_data_model_mongo.date_activate_promo_code,
            percentage_discount: promo_code_data_model_mongo.percentage_discount,
            free_shipping: promo_code_data_model_mongo.free_shipping,
            rule: (&promo_code_data_model_mongo.rule).into(),
        }
    }
}