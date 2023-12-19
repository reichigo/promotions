use async_trait::async_trait;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::Error;

#[async_trait]
pub trait TCreatePromoCodeUseCase{
    async fn execute(&self, promo_code: PromoCode) -> Result<PromoCode, Error>;
}