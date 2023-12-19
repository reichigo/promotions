use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::Error;
use crate::domain::repositories::TPromoCodeRepository;
use crate::use_case::t_get_promo_codes_user_applied_user_case::TGetPromoCodesUserAppliedUserCase;

pub struct GetPromoCodesUserAppliedUserCase<PCR: TPromoCodeRepository>{
    promo_code_repository: PCR
}

impl<PCR: TPromoCodeRepository> GetPromoCodesUserAppliedUserCase<PCR>{
    pub fn new(promo_code_repository: PCR) -> Self{
        Self{
            promo_code_repository
        }
    }
}

#[async_trait]
impl<PCR: TPromoCodeRepository + Send + Sync> TGetPromoCodesUserAppliedUserCase for GetPromoCodesUserAppliedUserCase<PCR>{
    async fn execute(&self, promo_code_id: Uuid, user_id: Uuid) -> Result<Option<UserPromotionsAppliedDto>, Error> {
        self.promo_code_repository.get_promotions_user_applied(promo_code_id, user_id).await
    }
}