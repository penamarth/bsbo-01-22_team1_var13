use crate::{Account, Delivery, Price, User};
use chrono::{DateTime, Utc};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[must_use]
pub struct Payment {
    pub id: Uuid,
    pub issue_date: DateTime<Utc>,
    pub payload: Price,
}
