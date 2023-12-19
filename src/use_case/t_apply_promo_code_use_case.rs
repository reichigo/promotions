use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::Error;

#[async_trait]
pub trait TApplyPromoCodeUseCase{
    async fn execute(&self, user_id: Uuid, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<(), Error>;
}