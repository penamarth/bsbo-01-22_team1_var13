#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not implemented")]
    Unimplemented,
}
