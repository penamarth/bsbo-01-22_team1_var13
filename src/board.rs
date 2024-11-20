use crate::{Account, Advertisement, AdvertisementStatus, Description, Item};
use chrono::Utc;
use itertools::Itertools;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Board {
    pub advertisements: Vec<Advertisement>,
    pub page_length: usize,
}

#[derive(Debug)]
#[must_use]
pub struct Query {
    pub search_string: String,
}

impl Board {
    #[instrument(skip_all, name = "load_board")]
    pub fn load() -> Self {
        let item_1 = Item::create("Advertisement #1", 500);
        let item_2 = Item::create("Advertisement #2", 1200);
        let description_1 = Description::create("Description of ad #1", vec![]);
        let description_2 = Description::create("Description of ad #2", vec![(), ()]);
        let ad_1 = Advertisement::create(item_1, description_1, Account::seller_1());
        let ad_2 = Advertisement::create(item_2, description_2, Account::seller_2());
        let mut board = Self {
            page_length: crate::PAGE_LENGTH,
            advertisements: vec![],
        };

        board.add_advertisement(ad_1);
        board.add_advertisement(ad_2);
        board
    }

    #[instrument(skip_all)]
    pub fn view_advertisements(&self) -> impl Iterator<Item = &Advertisement> {
        self.advertisements
            .iter()
            .sorted_by_key(|ad| ad.published_at)
            .rev()
            .take(self.page_length)
    }

    #[instrument(skip_all)]
    pub fn search_advertisements<'q>(
        &'q self,
        query: &'q Query,
    ) -> impl Iterator<Item = &'q Advertisement> {
        self.advertisements
            .iter()
            .sorted_by_key(|ad| ad.published_at)
            .rev()
            .filter(|ad| {
                let pattern = query.search_string.to_lowercase();
                let title_match = ad.item.title.to_lowercase().contains(&pattern);
                let body_match = ad.description.body.to_lowercase().contains(&pattern);
                title_match || body_match
            })
            .take(self.page_length)
    }

    #[instrument(skip_all)]
    pub fn add_advertisement(&mut self, advertisement: Advertisement) {
        self.advertisements.push(advertisement);
    }
}
