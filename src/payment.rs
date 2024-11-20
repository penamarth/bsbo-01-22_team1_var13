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

impl Payment {
    #[instrument(name = "request_payment", skip_all)]
    pub fn request_for(delivery: Delivery) -> (Delivery, Self) {
        let id = Uuid::new_v4();
        tracing::info!(message = "processing payment via PaymentAdapter", ?id);
        let payment = Self {
            id,
            issue_date: Utc::now(),
            payload: delivery.total_price(),
        };
        (delivery, payment)
    }
}
