use crate::Advertisement;
use itertools::Itertools;

pub type AdvertisementID = uuid::Uuid;

#[must_use]
pub struct Board {
    advertisement_ids: Vec<AdvertisementID>,
}

#[must_use]
pub struct Query {}

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
    pub fn search(query: &Query) -> Vec<Advertisement> {
        let _ = query;
        unimplemented!()
    }
}
