use async_trait::async_trait;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::entities::purchase::Purchase;

#[async_trait]
pub trait TCanApplyPromotionUseCase{
    async fn can_apply(purchase : &Purchase,
                       promo_code: &PromoCode,
                       user_promotions_applied_dto: UserPromotionsAppliedDto,
                       is_new_user: bool)
        -> (bool, String);

}