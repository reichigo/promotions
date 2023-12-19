use uuid::Uuid;

pub struct UserPromotionsAppliedDto{
    pub user_id: Uuid,
    pub promotions: Option<Vec<Uuid>>
}