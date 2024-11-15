#![expect(private_bounds, reason = "Intentionally sealed traits")]

use crate::{Advertisement, Board, Error, Query};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug)]
#[must_use]
pub struct Account<R: Role> {
    pub uuid: AcccountID,
    role: PhantomData<R>,
}

pub type AcccountID = Uuid;

trait Role {}
pub struct Guest;
pub struct User;
pub struct Moderator;
impl Role for Guest {}
impl Role for User {}
impl Role for Moderator {}

impl Account<Guest> {
    pub const fn unauthenticated() -> Self {
        Self {
            uuid: Uuid::nil(),
            role: PhantomData,
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
}

impl Account<Moderator> {
    pub fn as_guest(&self) -> Self {
        unimplemented!()
    }
}

impl<R: Role> Account<R> {
    #[must_use]
    pub fn view_board(&self) -> Vec<Advertisement> {
        Board::load().view_advertisements(crate::PAGE_LENGTH)
    }

    #[must_use]
    pub fn search_board(&self, search_string: &str) -> Vec<Advertisement> {
        let query = Query {
            search_string: search_string.to_owned(),
        };
        Board::load().search(&query)
    }
}
