use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromotionsAppliedModelMongo{
   pub user_id: bson::Uuid,
   pub promotions_id: Option<Vec<bson::Uuid>>
}