use std::result::Result as StdResult;
use thiserror::Error as ThisError;

/// Common error type to be used in the crate.
pub type Result<T> = StdResult<T, Error>;

/// Wrapper type for the different kinds of errors.
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Transaction format is invalid.")]
    InvalidTransactionFormat,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
