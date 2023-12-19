use uuid::Uuid;
use crate::db::datasource::mongo::data_model::user_promotions_applied_model_mongo::UserPromotionsAppliedModelMongo;
use crate::domain::dto::user_promotions_applied_dto::UserPromotionsAppliedDto;

impl From<UserPromotionsAppliedModelMongo> for UserPromotionsAppliedDto{
    fn from(user_promotions_applied_model_mongo: UserPromotionsAppliedModelMongo) -> Self {



        Self{
            user_id : Uuid::parse_str(&user_promotions_applied_model_mongo.user_id.to_string()).unwrap(),
            promotions: match user_promotions_applied_model_mongo.promotions_id {
                None => None,
                Some(promotions) => Some(promotions.iter().map(|id| Uuid::parse_str(&id.to_string()).unwrap()).collect())
            }
        }
    }
}