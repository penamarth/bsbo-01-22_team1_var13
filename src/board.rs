use crate::{Account, Advertisement, Delivery, DeliveryStatus, Description, Item};
use crate::{AdvertisementStatus, Moderator, Payment, User};
use chrono::Utc;
use itertools::Itertools;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Board {
    advertisements: Vec<Advertisement>,
    users: Vec<Account<User>>,
    moderators: Vec<Account<Moderator>>,
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
        let test_user = Account::test_user();
        let test_seller = Account::test_seller();
        let ad_1 = Advertisement::create(item_1, description_1, test_user.clone());
        let ad_2 = Advertisement::create(item_2, description_2, test_seller.clone());
        let mut board = Self::default();

        board.users.extend([test_user, test_seller]);
        board.add_advertisement(ad_1);
        board.add_advertisement(ad_2);
        board
    }

    #[must_use]
    pub fn get_user_mut(&mut self, user_uuid: Uuid) -> Option<&mut Account<User>> {
        self.users.iter_mut().find(|user| user.uuid == user_uuid)
    }

    #[must_use]
    pub fn get_moderator_mut(&mut self, moderator_uuid: Uuid) -> Option<&mut Account<Moderator>> {
        self.moderators
            .iter_mut()
            .find(|user| user.uuid == moderator_uuid)
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

    #[instrument(skip_all)]
    pub fn extend_cart(
        &mut self,
        user_uuid: Uuid,
        items: impl IntoIterator<Item = Item>,
    ) -> Result<(), crate::Error> {
        let user = self
            .get_user_mut(user_uuid)
            .ok_or(crate::Error::UserNotFound(user_uuid))?;
        user.cart.items.extend(items);
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn place_order(&mut self, user_uuid: Uuid) -> Result<(), crate::Error> {
        let user = self
            .get_user_mut(user_uuid)
            .ok_or(crate::Error::UserNotFound(user_uuid))?;
        let delivery = std::mem::replace(&mut user.cart, Delivery::blank());
        let (mut paid_delivery, payment) = Payment::request_for(delivery);
        tracing::info!(message = "saving paid order for", ?user_uuid);
        paid_delivery.update_status(DeliveryStatus::Collecting);
        paid_delivery.update_status(DeliveryStatus::InTransit);
        paid_delivery.update_status(DeliveryStatus::AwaitsPickup);
        user.past_orders.insert(paid_delivery, payment);
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            advertisements: vec![],
            page_length: crate::PAGE_LENGTH,
            users: vec![],
            moderators: vec![],
        }
    }
}
