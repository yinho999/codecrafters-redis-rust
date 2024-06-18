use crate::error::Error;
pub mod handler;
pub mod error;
mod parser;
pub mod app;

pub type Result<T> = std::result::Result<T,Error>;