use async_trait::async_trait;
use uuid::Uuid;
use crate::db::datasource::mongo::TDataSourcePromoCodeMongo;
use crate::domain::entities::promo_code::PromoCode;
use crate::domain::{Error};
use crate::domain::dto::PromoCodeDto;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;
use crate::domain::repositories::TPromoCodeRepository;

pub struct PromoCodeRepository<PMM: TDataSourcePromoCodeMongo>{
    data_source_promo_code_mongo: PMM
}

impl<PMM: TDataSourcePromoCodeMongo> PromoCodeRepository<PMM>{
    pub fn new(data_source_promo_code_mongo: PMM) -> Self{
        Self{
            data_source_promo_code_mongo
        }
    }
}

#[async_trait]
impl<PMM: TDataSourcePromoCodeMongo + Send + Sync> TPromoCodeRepository for PromoCodeRepository<PMM> {
    async fn create(&self, promo_code: &PromoCode) -> Result<(), Error> {
        self.data_source_promo_code_mongo.create(&promo_code).await
    }

    async fn apply_promo_code(&self, user_id: Uuid, promo_code_id: Uuid) -> Result<(), Error> {
        self.data_source_promo_code_mongo.apply_promo_code(user_id, promo_code_id).await
    }

    async fn get_promo_code(&self, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<Option<PromoCodeDto>, Error> {
        self.data_source_promo_code_mongo.get_promo_code(promo_code_id, promo_code_name).await
    }

    async fn get_promotions_user_applied(&self, promo_code_id: Uuid, user_id: Uuid) -> Result<Option<UserPromotionsAppliedDto>, Error> {
        self.get_promotions_user_applied(promo_code_id, user_id).await
    }
}