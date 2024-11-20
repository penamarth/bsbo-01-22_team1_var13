use crate::{Account, Delivery, User};
use chrono::{DateTime, Utc};

#[derive(Debug)]
#[must_use]
pub struct Payment {
    pub buyer: Account<User>,
    pub delivery: Delivery,
    pub issue_date: DateTime<Utc>,
}
