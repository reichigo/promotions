use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::dto::PromoCodeDto;
use crate::domain::Error;

#[async_trait]
pub trait TGetPromoCodeUseCase{
    async fn execute(&self, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<Option<PromoCodeDto>, Error>;
}