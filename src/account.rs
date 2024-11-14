#![expect(private_bounds, reason = "Intentionally sealed traits")]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![allow(clippy::result_unit_err)]

use std::marker::PhantomData;
use uuid::Uuid;

#[must_use]
pub struct Account<R: Role> {
    pub uuid: Uuid,
    role: PhantomData<R>,
}

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
    pub fn authenticate_as_user(self) -> Result<Account<User>, ()> {
        unimplemented!()
    }

    pub fn authenticate_as_moderator(self) -> Result<Account<Moderator>, ()> {
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
