use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not implemented")]
    Unimplemented,

    #[error("User with UUID {0:?} not found")]
    UserNotFound(Uuid),

    #[error("Cannot form an order with no items")]
    EmptyCart,
}
