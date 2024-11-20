use crate::Error;
use std::num::NonZeroUsize;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[must_use]
pub struct Description {
    pub id: Uuid,
    pub body: String,
    pub images: Vec<Image>,
}

pub type Image = ();

impl Description {
    #[instrument(name = "create_description", skip(body))]
    pub fn create(body: &str, images: Vec<Image>) -> Self {
        Self {
            id: Uuid::new_v4(),
            body: body.to_string(),
            images,
        }
    }
}
