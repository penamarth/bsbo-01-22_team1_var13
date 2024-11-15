use crate::Error;
use std::num::NonZeroUsize;
use uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Description {
    pub id: DescriptionID,
    pub body: String,
    pub images: Vec<Image>,
}

pub type DescriptionID = Uuid;
pub type Image = ();

impl Description {
    pub fn load(id: DescriptionID) -> Result<Self, Error> {
        let _ = id;
        unimplemented!()
    }
}
