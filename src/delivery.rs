use crate::{Account, Item, Payment, Price, User};
use tracing::instrument;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[must_use]
pub struct Delivery {
    pub items: Vec<Item>,
    pub destination: Destination,
}

pub type Destination = ();

impl Delivery {
    pub const fn blank() -> Self {
        Self {
            items: vec![],
            destination: (),
        }
    }

    #[must_use]
    pub fn total_price(&self) -> Price {
        self.items.iter().map(|item| item.price).sum()
    }
}
