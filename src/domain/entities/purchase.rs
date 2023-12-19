use uuid::Uuid;

pub struct Purchase{
    total_amount: Option<f64>,
    purchase_items: Option<Vec<Uuid>>
}

impl Purchase{
    pub fn new (total_amount: Option<f64>,
                purchase_items: Option<Vec<Uuid>>) -> Self{
        Self{
            total_amount,
            purchase_items
        }
    }
    pub fn total_amount(&self) -> Option<f64> {
        self.total_amount
    }
    pub fn purchase_items(&self) -> Option<&Vec<Uuid>> {
        self.purchase_items.as_ref()
    }
}