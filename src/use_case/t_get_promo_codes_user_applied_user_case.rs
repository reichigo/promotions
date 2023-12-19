use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::Error;

#[async_trait]
pub trait TGetPromoCodesUserAppliedUserCase{
        async fn execute(&self, promo_code_id: Uuid, user_id: Uuid) -> Result<Option<UserPromotionsAppliedDto>, Error>;
}