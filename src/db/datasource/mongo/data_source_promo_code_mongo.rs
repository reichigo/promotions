use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::{doc, Document};
use uuid::Uuid;
use crate::db::datasource::mongo::data_model::PromoCodeDataModelMongo;
use crate::db::datasource::mongo::data_model::user_promotions_applied_model_mongo::{UserPromotionsAppliedModelMongo};
use crate::db::datasource::mongo::TDataSourcePromoCodeMongo;
use crate::db::MongoClient;

use crate::domain::entities::promo_code::PromoCode;
use crate::domain::{Error};
use crate::domain::dto::PromoCodeDto;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;

pub struct  DataSourcePromoCodeMongo;
static COLLECTION: &str = "promotions";

impl DataSourcePromoCodeMongo{
    pub fn new() -> Self{
        Self{}
    }
}

#[async_trait]
impl TDataSourcePromoCodeMongo for DataSourcePromoCodeMongo{
    async fn create(&self, promo_code: &PromoCode) -> Result<(), Error> {
        let mongo_collection = MongoClient::get_collection::<PromoCodeDataModelMongo>(COLLECTION).await;
        let promo_code_mongo: PromoCodeDataModelMongo = promo_code.into();

        mongo_collection
            .insert_one(promo_code_mongo, None)
            .await
            .map(|_| ())
            .map_err(|e| Error::internal_server_error(e.to_string()))
    }

    async fn apply_promo_code(&self, user_id: Uuid, promo_code_id: Uuid) -> Result<(), Error> {
        let mongo_collection_user_promotions = MongoClient::get_collection::<UserPromotionsAppliedModelMongo>(COLLECTION).await;
        let mongo_collection_promotions = MongoClient::get_collection::<PromoCodeDataModelMongo>(COLLECTION).await;
        let user_id = bson::Uuid::parse_str(user_id.to_string()).unwrap();
        let promo_code_id = bson::Uuid::parse_str(promo_code_id.to_string()).unwrap();

        mongo_collection_user_promotions
            .update_one(doc! {"_id": user_id}, doc!{"$set": { "promotions_applied.$.promotion_id": promo_code_id }}, None)
            .await
            .map_err(|e| Error::internal_server_error(e.to_string()))?;

        mongo_collection_promotions
            .update_one(doc! {"_id": promo_code_id}, doc!{"$set": { "users_promotion_applied.$.user_id": user_id }}, None)
            .await
            .map_err(|e| Error::internal_server_error(e.to_string()))
            .map(|_|())
    }

    async fn get_promo_code(&self, promo_code_id: Option<Uuid>, promo_code_name: Option<String>) -> Result<Option<PromoCodeDto>, Error> {
        let mongo_collection_promotions = MongoClient::get_collection::<PromoCodeDataModelMongo>(COLLECTION).await;
        let filter: Document;

        if promo_code_id != None{
            filter = doc! {"_id": bson::Uuid::parse_str(promo_code_id.unwrap().to_string()).unwrap()};
        }
        else {
            filter = doc! {"name": bson::Uuid::parse_str(promo_code_name.unwrap().to_string()).unwrap()};
        }

       let promo_code_mongo_option = mongo_collection_promotions
            .find_one(filter, None)
            .await
            .map_err(|e| Error::internal_server_error(e.to_string()))?;

        Ok(match promo_code_mongo_option {
            None => None,
            Some(promo_code_mongo) =>  Some(PromoCodeDto::from(&promo_code_mongo))
        })
    }

    async fn get_promotions_user_applied(&self, promo_code_id: Uuid, user_id: Uuid) -> Result<Option<UserPromotionsAppliedDto>, Error> {
        let mongo_collection_user_promotions = MongoClient::get_collection::<UserPromotionsAppliedModelMongo>(COLLECTION).await;

        let query = doc! {"_id": bson::Uuid::parse_str(promo_code_id.to_string()).unwrap()};

        let user_promo_code_option = mongo_collection_user_promotions
            .find_one(query, None)
            .await
            .map_err(|e| Error::internal_server_error(e.to_string()))?;

        Ok(match user_promo_code_option {
            None => None,
            Some(user_promotions) =>  Some(UserPromotionsAppliedDto::from(user_promotions))
        })
    }
}