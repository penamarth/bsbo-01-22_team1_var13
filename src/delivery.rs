use crate::{Account, Item, Payment, Price, User};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::collections::HashMap;
use tracing::instrument;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[must_use]
pub struct Delivery {
    pub items: Vec<Item>,
    pub statuses: Vec<(DeliveryStatus, DateTime<Utc>)>,
    pub destination: Destination,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, strum::Display)]
pub enum DeliveryStatus {
    Collecting,   // "Собираем заказ"
    InTransit,    // "Доставляем"
    AwaitsPickup, // "Ожидает получения в ПВЗ"
    Done,         // "Получен"
}

pub type Destination = ();

impl Delivery {
    pub const fn blank() -> Self {
        Self {
            items: vec![],
            statuses: vec![],
            destination: (),
        }
    }

    #[instrument(name = "get_total_price", skip_all)]
    #[must_use]
    pub fn total_price(&self) -> Price {
        self.items.iter().map(|item| item.price).sum()
    }

    #[instrument(name = "update_delivery_status", skip_all)]
    pub fn update_status(&mut self, new_status: DeliveryStatus) {
        tracing::info!(message = "updating status", ?new_status);
        self.statuses.push((new_status, Utc::now()));
    }

    #[instrument(name = "track_delivery", skip_all)]
    pub fn track(&self) -> impl Iterator<Item = &(DeliveryStatus, DateTime<Utc>)> {
        tracing::info!(message = "collecting all previous statuses");
        self.statuses
            .iter()
            .sorted_unstable_by_key(|(_, datetime)| datetime)
            .rev()
    }
}
