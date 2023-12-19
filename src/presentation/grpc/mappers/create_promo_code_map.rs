use chrono::{TimeZone, Utc};
use uuid::Uuid;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::Error;
use crate::presentation::grpc::generated::{CreatePromoCodeRequest, CreatePromoCodeResponse};

impl TryFrom<&CreatePromoCodeRequest> for PromoCode{
    
    type Error = Error;
    fn try_from(create_promo_code_request: &CreatePromoCodeRequest) -> Result<Self, Error> {

        let expiration_date = match &create_promo_code_request.expiration_date {
            None => None,
            Some(value) => Some(Utc.timestamp_opt(value.seconds, 0).unwrap())
        };

        if create_promo_code_request.date_activate_promo_code.is_none(){
            return Err(Error::bad_request("date activate promo code can't be null".to_string()))
        }

        let date_activate_promo_code = Utc.timestamp_opt(create_promo_code_request.date_activate_promo_code.as_ref().unwrap().seconds, 0).unwrap();

        let items_promotions = match create_promo_code_request.items_promotion.len() > 0 {
            true => Some(create_promo_code_request.items_promotion.iter().map(|item_id| Uuid::parse_str(item_id).unwrap()).collect::<Vec<Uuid>>()),
            false => None
        };

        Ok(PromoCode::new(
            create_promo_code_request.name.to_string(),
            create_promo_code_request.total_promo_code,
            expiration_date,
            create_promo_code_request.quantity_per_customer,
            create_promo_code_request.percentage_discount,
            create_promo_code_request.total_money_discount,
            date_activate_promo_code,
            create_promo_code_request.free_shipping,
            items_promotions ,
            create_promo_code_request.rule.as_ref().unwrap().into(),
        ))
    }
}

impl From<&PromoCode> for CreatePromoCodeResponse{
    fn from(promo_code: &PromoCode) -> Self {
        Self{
            id_promotion_created: promo_code.id().to_string(),
            name: promo_code.name().to_string(),
            quantity_per_customer: promo_code.quantity_per_customer(),
            expiration_date: match promo_code.expiration_date() {
                None => None,
                Some(date) => Some(prost_types::Timestamp{
                    seconds: date.timestamp(),
                    nanos: 0,
                })
            },
            total_promo_code: promo_code.total_promo_code(),
            percentage_discount: promo_code.percentage_discount(),
            total_money_discount: Some(33.9),
            free_shipping: promo_code.free_shipping(),
            date_activate_promo_code: match promo_code.expiration_date() {
                None => None,
                Some(date) => Some(prost_types::Timestamp{
                    seconds: date.timestamp(),
                    nanos: 0
                })
            },
        }
    }
}