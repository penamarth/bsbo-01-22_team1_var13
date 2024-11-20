use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub price: Price,
}

pub type Price = u64;

impl Item {
    #[instrument(name = "create_item")]
    pub fn create(title: &str, price: Price) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            price,
        }
    }
}
