use async_trait::async_trait;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::Error;
use crate::domain::repositories::TPromoCodeRepository;
use crate::use_case::TCreatePromoCodeUseCase;

pub struct  CreatePromoCodeUseCase<PR: TPromoCodeRepository>{
    promo_code_repository : PR
}

impl<PR: TPromoCodeRepository + Send + Sync> CreatePromoCodeUseCase<PR>{
    pub fn new (promo_code_repository : PR) -> Self{
        Self{
            promo_code_repository
        }
    }
}

#[async_trait]
impl <PR: TPromoCodeRepository + Send + Sync> TCreatePromoCodeUseCase for CreatePromoCodeUseCase<PR>{
    async fn execute(&self, promo_code: PromoCode) -> Result<PromoCode, Error> {
        self.promo_code_repository.create(&promo_code).await?;

        Ok(promo_code)
    }
}

