use crate::{Account, Description, Item, Moderator, User};
use chrono::prelude::*;
use color_eyre::owo_colors::OwoColorize;
use std::fmt;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Clone)]
#[must_use]
pub struct Advertisement {
    pub id: Uuid,
    pub status: AdvertisementStatus,
    pub published_at: DateTime<Utc>,
    pub item: Item,
    pub description: Description,
    pub seller: Account<User>,
}

#[derive(Debug, Clone)]
pub enum AdvertisementStatus {
    Listed,
    AwaitsModeration { moderator: Account<Moderator> },
}

impl Advertisement {
    #[instrument(skip_all, name = "create_advertisement")]
    pub fn create(item: Item, description: Description, seller: Account<User>) -> Self {
        Self {
            id: Uuid::new_v4(),
            published_at: Utc::now(),
            status: AdvertisementStatus::AwaitsModeration {
                moderator: Account::random_moderator(),
            },
            item,
            description,
            seller,
        }
    }

    #[instrument(skip_all)]
    pub fn confirm_moderation(&mut self) {
        self.status = AdvertisementStatus::Listed;
    }
}
