use chrono::prelude::*;
use uuid::Uuid;

use crate::DescriptionID;

#[derive(Debug)]
#[must_use]
pub struct Advertisement {
    pub id: Uuid,
    pub status: Status,
    pub published_at: DateTime<Utc>,
    pub description_id: DescriptionID,
}

#[derive(Debug)]
pub enum Status {
    Listed,
    AwaitsModeration { moderator: crate::AcccountID },
}

impl Advertisement {
    pub fn load(id: &Uuid) -> Self {
        Self {
            id: *id,
            description_id: Uuid::new_v4(),
            published_at: DateTime::UNIX_EPOCH,
            status: Status::AwaitsModeration {
                moderator: Uuid::new_v4(),
            },
        }
    }
}
