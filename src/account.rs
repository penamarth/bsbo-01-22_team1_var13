#![expect(private_bounds, reason = "Intentionally sealed traits")]

use crate::{Advertisement, Board, Delivery, Error, Payment, Query};
use std::{collections::HashMap, marker::PhantomData};
use uuid::{uuid, Uuid};

#[derive(Debug, Clone)]
#[must_use]
pub struct Account<R: Role> {
    pub uuid: AcccountID,
    pub name: String,
    pub cart: Delivery,
    pub past_orders: HashMap<Delivery, Payment>,
    role: PhantomData<R>,
}

pub type AcccountID = Uuid;

trait Role {}
#[derive(Debug, Clone, Copy)]
pub struct Guest;
#[derive(Debug, Clone, Copy)]
pub struct User;
#[derive(Debug, Clone, Copy)]
pub struct Moderator;
impl Role for Guest {}
impl Role for User {}
impl Role for Moderator {}

impl Account<Guest> {
    pub fn unauthenticated() -> Self {
        Self {
            uuid: Uuid::nil(),
            role: PhantomData,
            name: String::from("guest"),
            cart: Delivery::blank(),
            past_orders: HashMap::new(),
        }
    }
}

impl Default for Account<Guest> {
    fn default() -> Self {
        Self::unauthenticated()
    }
}

impl Account<Guest> {
    pub fn authenticate_as_user(self) -> Result<Account<User>, Error> {
        unimplemented!()
    }

    pub fn authenticate_as_moderator(self) -> Result<Account<Moderator>, Error> {
        unimplemented!()
    }
}

impl Account<User> {
    pub fn as_guest(&self) -> Self {
        unimplemented!()
    }

    pub const TEST_USER_UUID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    pub const TEST_SELLER_UUID: Uuid = uuid!("a9a65d98-25f6-48f7-89da-5e69691acaea");

    pub fn test_user() -> Self {
        Self {
            uuid: Self::TEST_USER_UUID,
            name: String::from("Kirill Butorin"),
            role: PhantomData,
            cart: Delivery::blank(),
            past_orders: HashMap::new(),
        }
    }

    pub fn test_seller() -> Self {
        Self {
            uuid: Self::TEST_SELLER_UUID,
            name: String::from("Maxim Ganshin"),
            ..Self::test_user()
        }
    }
}

impl Account<Moderator> {
    pub fn as_guest(&self) -> Self {
        unimplemented!()
    }

    pub fn random_moderator() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: String::from("Moderator"),
            role: PhantomData,
            cart: Delivery::blank(),
            past_orders: HashMap::new(),
        }
    }
}
