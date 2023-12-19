use async_trait::async_trait;
use crate::domain::dto::PromoCodeDto;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::entities::purchase::Purchase;
use crate::use_case::t_can_apply_promotion_use_case::TCanApplyPromotionUseCase;

pub struct CanApplyPromotionUseCase;

#[async_trait]
impl TCanApplyPromotionUseCase for CanApplyPromotionUseCase{
    async fn can_apply(purchase : &Purchase, promo_code: &PromoCode, user_promotions_applied_dto: UserPromotionsAppliedDto, is_new_user: bool) -> (bool, String) {


        if let (false, message) = promo_code.is_promo_code_valid(purchase, promo_code, is_new_user) {
            return (false, message)
        }

        (true, String::new())
    }
}

impl CanApplyPromotionUseCase{
    fn is_new_user_valid(promo_code_dto: &PromoCodeDto, is_new_user: Option<bool>) -> (bool, String) {

        let (Some(is_for_new_user), Some(is_new_user_status)) = (promo_code_dto.rule.is_for_new_user, is_new_user) else {
            return  (true, String::default());
        };

        if is_for_new_user &&
            !is_new_user_status {
            return (false, String::from("This promotion is just for new users"));
        }

        (true, String::new())
    }
}