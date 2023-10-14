#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error)
}