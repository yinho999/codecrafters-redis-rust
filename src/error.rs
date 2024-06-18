use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    IO(#[from] std::io::Error),
    TokioTaskError(#[from] tokio::task::JoinError),
    Multiple(Vec<Error>),
    Unknown,
}



impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => {
                write!(formatter, "IO error: {e}", )
            }
            Self::TokioTaskError(e) => {
                write!(formatter, "Tokio task error: {e}", )
            }
            Self::Multiple(errors) => {
                write!(formatter, "{}", &format!("Multiple errors: {errors:?}",))
            }
            Self::Unknown => {
                write!(formatter, "Unknown error")
            }
        }
    }
}