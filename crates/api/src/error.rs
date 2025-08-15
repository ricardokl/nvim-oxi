use std::fmt::Display;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, ThisError, Eq, PartialEq)]
pub enum Error {
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Nvim(#[from] types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] types::conversion::Error),

    #[error("{0}")]
    Message(String),
}

impl<S: Into<String>> From<S> for Error {
    fn from(s: S) -> Self {
        Self::Message(s.into())
    }
}
