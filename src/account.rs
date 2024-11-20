#![expect(private_bounds, reason = "Intentionally sealed traits")]

use crate::{Advertisement, Board, Delivery, Error, Payment, Query};
use std::{collections::HashMap, marker::PhantomData};
use uuid::{uuid, Uuid};

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Guest;
#[derive(Debug)]
pub struct User;
#[derive(Debug)]
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

    pub fn seller_1() -> Self {
        Self {
            uuid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            name: String::from("Kirill Butorin"),
            role: PhantomData,
            cart: Delivery::blank(),
            past_orders: HashMap::new(),
        }
    }

    pub fn seller_2() -> Self {
        Self {
            uuid: uuid!("a9a65d98-25f6-48f7-89da-5e69691acaea"),
            name: String::from("Maxim Ganshin"),
            ..Self::seller_1()
        }
    }
}

impl Account<Moderator> {
    pub fn as_guest(&self) -> Self {
        unimplemented!()
    }

    pub fn moderator() -> Self {
        Self {
            uuid: uuid!("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4"),
            name: String::from("Moderator"),
            role: PhantomData,
            cart: Delivery::blank(),
            past_orders: HashMap::new(),
        }
    }
}
