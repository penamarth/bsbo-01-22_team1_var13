use chrono::prelude::*;
use uuid::Uuid;

#[must_use]
pub struct Advertisement {
    pub id: Uuid,
    pub published_at: DateTime<Utc>,
}

impl Advertisement {
    pub const fn load(id: &Uuid) -> Self {
        Self {
            id: *id,
            published_at: DateTime::UNIX_EPOCH,
        }
    }
}
