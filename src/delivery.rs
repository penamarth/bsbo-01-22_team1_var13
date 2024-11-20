use crate::{Account, Item, User};

#[derive(Debug)]
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
}
