use uuid::Uuid;
use crate::domain::entities::promo_code::PromoCode;

pub struct User{
    id: Uuid,
    promo_codes_applied: Vec<PromoCode>
}