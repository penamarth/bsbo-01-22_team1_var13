use crate::Advertisement;
use itertools::Itertools;

pub type AdvertisementID = uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Board {
    advertisement_ids: Vec<AdvertisementID>,
}

#[derive(Debug)]
#[must_use]
pub struct Query {
    pub search_string: String,
}

impl Board {
    pub const fn load() -> Self {
        Self {
            advertisement_ids: vec![],
        }
    }

    #[must_use]
    pub fn view_advertisements(&self, count: usize) -> Vec<Advertisement> {
        self.advertisement_ids
            .iter()
            .map(Advertisement::load)
            .sorted_by_key(|ad| ad.published_at)
            .take(count)
            .collect()
    }

    #[must_use]
    pub fn search(&self, query: &Query) -> Vec<Advertisement> {
        let _ = query;
        unimplemented!()
    }
}
