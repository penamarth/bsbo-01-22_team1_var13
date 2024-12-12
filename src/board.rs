use crate::{Account, Advertisement, Delivery, DeliveryStatus, Description};
use crate::{AdvertisementStatus, Item, Moderator, Payment, User};
use crate::{ExternalPaymentSystem, PaymentAdapter, PaymentSystem};
use bon::{builder, Builder};
use chrono::Utc;
use itertools::Itertools;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Builder)]
#[builder(start_fn = builder)]
#[must_use]
pub struct Board<P: PaymentSystem> {
    advertisements: Vec<Advertisement>,
    users: Vec<Account<User>>,
    moderators: Vec<Account<Moderator>>,
    payment_adapter: PaymentAdapter<P>,
    pub page_length: usize,
}

#[derive(Debug, Builder)]
#[builder(start_fn = new)]
#[must_use]
pub struct Query {
    pub search_string: String,
}

impl Board<ExternalPaymentSystem> {
    #[instrument(skip_all, name = "load_board")]
    pub fn load() -> Result<Self, crate::Error> {
        let item_1 = Item::create("Advertisement #1", 500);
        let item_2 = Item::create("Advertisement #2", 1200);
        let description_1 = Description::create("Description of ad #1", vec![]);
        let description_2 = Description::create("Description of ad #2", vec![(), ()]);
        let test_user = Account::test_user();
        let test_seller = Account::test_seller();
        let ad_1 = Advertisement::create(item_1, description_1, test_user.clone());
        let ad_2 = Advertisement::create(item_2, description_2, test_seller.clone());
        let payment_system = ExternalPaymentSystem::from_env()?;
        Ok(Self::builder()
            .page_length(crate::PAGE_LENGTH)
            .advertisements(vec![ad_1, ad_2])
            .moderators(vec![])
            .users(vec![test_user, test_seller])
            .payment_adapter(PaymentAdapter::for_payment_system(payment_system))
            .build())
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
        let delivery = {
            let user = self
                .get_user_mut(user_uuid)
                .ok_or(crate::Error::UserNotFound(user_uuid))?;
            tracing::info!(message = "checking user's cart for items");
            if user.cart.items.is_empty() {
                return Err(crate::Error::EmptyCart);
            }
            std::mem::replace(&mut user.cart, Delivery::blank())
        };

        let (mut paid_delivery, payment) = self /* <-- self это Board */
            .payment_adapter // Обращаемся к адаптеру.
            .payment_system // Обращаемся к платежной системе (внутри адаптера могут быть другие сервисы).
            .request_payment(delivery); // Запрашиваем оплату для заказа.

        tracing::info!(message = "saving paid order", buyer = ?user_uuid);
        paid_delivery.update_status(DeliveryStatus::Collecting);
        paid_delivery.update_status(DeliveryStatus::InTransit);
        paid_delivery.update_status(DeliveryStatus::AwaitsPickup);

        let user = self
            .get_user_mut(user_uuid)
            .ok_or(crate::Error::UserNotFound(user_uuid))?;
        user.past_orders.insert(paid_delivery, payment);

        Ok(())
    }
}
