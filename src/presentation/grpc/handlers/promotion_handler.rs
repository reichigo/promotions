use tonic::{Request, Response, Status};
use crate::cross_cutting::CastCodeStatus;
use crate::domain::entities::promo_code::PromoCode;
use crate::presentation::grpc::generated::{ApplyPromoCodeRequest, ApplyPromoCodeResponse, CreatePromoCodeRequest, CreatePromoCodeResponse, GetGeneralPromotionsRequest, GetGeneralPromotionsResponse, GetUserPromotionsRequest, GetUserPromotionsResponse};
use crate::presentation::grpc::generated::promotion_service_server::PromotionService;
use crate::use_case::TCreatePromoCodeUseCase;

pub struct PromotionHandler<PUS: TCreatePromoCodeUseCase>{
     create_promo_code_use_case: PUS
}

impl<PUS: TCreatePromoCodeUseCase + Send + Sync> PromotionHandler<PUS>{
    pub fn new(create_promo_code_use_case: PUS) -> PromotionHandler<PUS>{

        PromotionHandler{
            create_promo_code_use_case
        }
    }
}

#[tonic::async_trait]
impl<PCR: TCreatePromoCodeUseCase + Send + Sync + 'static> PromotionService for PromotionHandler<PCR>{
    async fn create_promo_code(&self, request: Request<CreatePromoCodeRequest>) -> Result<Response<CreatePromoCodeResponse>, Status> {
        let promo_code = PromoCode::try_from(request.get_ref())
            .map_err(|e| CastCodeStatus::cast_to_tonic_status(e.status_code, e.message))?;

            self.create_promo_code_use_case.execute(promo_code)
            .await
            .map_err(|e| CastCodeStatus::cast_to_tonic_status(e.status_code, e.message))
            .map(|pc| Response::new((&pc).into()))
    }

    async fn apply_promo_code(&self, request: Request<ApplyPromoCodeRequest>) -> Result<Response<ApplyPromoCodeResponse>, Status> {
        todo!()
    }

    async fn get_user_promotions(&self, request: Request<GetUserPromotionsRequest>) -> Result<Response<GetUserPromotionsResponse>, Status> {
        todo!()
    }

    async fn get_general_promotions(&self, request: Request<GetGeneralPromotionsRequest>) -> Result<Response<GetGeneralPromotionsResponse>, Status> {
        todo!()
    }
}