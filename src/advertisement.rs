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
}

impl fmt::Display for Advertisement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            self.item.title.bold().underline(),
            match &self.status {
                AdvertisementStatus::Listed => "",
                AdvertisementStatus::AwaitsModeration { .. } => "(awaits moderation)",
            }
            .bright_black()
        )?;

        writeln!(f, "{} {}", "Description:".bold(), self.description.body)?;
        writeln!(
            f,
            "{} ({} images)",
            "Images:".bold(),
            self.description.images.len()
        )?;
        writeln!(f, "{} {}", "Price:".bold(), self.item.price.yellow())?;

        match &self.status {
            AdvertisementStatus::Listed => write!(f, "Listed by {}", self.seller.name.blue())?,
            AdvertisementStatus::AwaitsModeration { moderator } => {
                write!(f, "Awaits moderation by {:?}", moderator.name)?;
            }
        }

        Ok(())
    }
}
