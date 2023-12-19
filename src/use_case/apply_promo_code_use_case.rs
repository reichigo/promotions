use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::Error;
use crate::use_case::t_apply_promo_code_use_case::TApplyPromoCodeUseCase;
use crate::use_case::t_get_promo_code_use_case::TGetPromoCodeUseCase;
use crate::use_case::t_get_promo_codes_user_applied_user_case::TGetPromoCodesUserAppliedUserCase;

pub struct ApplyPromoCodeUseCase<GPCUS: TGetPromoCodeUseCase, GUP: TGetPromoCodesUserAppliedUserCase>{
    get_promo_code_use_case: GPCUS,
    get_promo_codes_user_applied_user_case: GUP
}

impl<GPCUS: TGetPromoCodeUseCase, GUP: TGetPromoCodesUserAppliedUserCase> ApplyPromoCodeUseCase<GPCUS, GUP>{
    pub fn new(get_promo_code_use_case: GPCUS, get_promo_codes_user_applied_user_case: GUP) -> Self{
        Self{
            get_promo_code_use_case,
            get_promo_codes_user_applied_user_case
        }
    }
}
#[async_trait]
impl<GPCUS: TGetPromoCodeUseCase + Send + Sync, GUP: TGetPromoCodesUserAppliedUserCase + Send + Sync> TApplyPromoCodeUseCase for ApplyPromoCodeUseCase<GPCUS, GUP>{
    async fn execute(&self, user_id: Uuid, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<(), Error> {

        if promo_code_id.is_none() && promo_code_name.is_none(){
           return  Err(Error::bad_request("promo_code_id' or 'promo_code_name' needs to be filled.".to_string()));
        }

        let promo_code = self.get_promo_code_use_case.execute(promo_code_id, promo_code_name).await?;
        let promo_code = promo_code.unwrap();
        let user_promotions_applied = self.get_promo_codes_user_applied_user_case.execute(promo_code.id, user_id).await?;

        Ok(())

    }
}