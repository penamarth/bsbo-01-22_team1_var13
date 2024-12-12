use std::env::VarError;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not implemented")]
    Unimplemented,

    #[error("User with UUID {0:?} not found")]
    UserNotFound(Uuid),

    #[error("Cannot form an order with no items")]
    EmptyCart,

    #[error("The external payment system is not specified")]
    PaymentSystemNotSet(#[from] VarError),

    #[error("The external payment system does not match any known ones")]
    UnknownPaymentSystem,
}
