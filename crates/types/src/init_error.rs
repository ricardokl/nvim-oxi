use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, Eq, PartialEq, Hash)]
pub enum InitError {
    #[error("The types arena has already been initialized")]
    ArenaAlreadyInitialized,
}
