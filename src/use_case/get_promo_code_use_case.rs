use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::dto::PromoCodeDto;
use crate::domain::Error;
use crate::domain::repositories::TPromoCodeRepository;
use crate::use_case::t_get_promo_code_use_case::TGetPromoCodeUseCase;

pub struct GetPromoCodeUseCase<PCR: TPromoCodeRepository>{
    promo_code_repository: PCR
}

impl<PCR: TPromoCodeRepository> GetPromoCodeUseCase<PCR>{
    pub fn new(promo_code_repository: PCR) -> Self{
        Self{
            promo_code_repository
        }
    }
}

#[async_trait]
impl<PCR: TPromoCodeRepository + Send + Sync> TGetPromoCodeUseCase for GetPromoCodeUseCase<PCR>{
    async fn execute(&self, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<Option<PromoCodeDto>, Error> {
        self.promo_code_repository.get_promo_code(promo_code_id, promo_code_name).await
    }
}
