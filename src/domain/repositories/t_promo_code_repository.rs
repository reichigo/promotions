use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::dto::PromoCodeDto;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::Error;

#[async_trait]
pub trait TPromoCodeRepository{
    async fn create(&self, promo_code: &PromoCode) -> Result<(), Error>;
    async fn apply_promo_code(&self, user_id: Uuid, promo_code_id: Uuid) -> Result<(), Error>;
    async fn get_promo_code(&self, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<Option<PromoCodeDto>, Error>;
    async fn get_promotions_user_applied(&self, promo_code_id: Uuid, user_id: Uuid) -> Result<Option<UserPromotionsAppliedDto>, Error>;
}