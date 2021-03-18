//! Pure Rust implementation of exact arithmetic and algorithms for computer geometry applications.

pub mod natural;
pub mod slate;

pub use crate::slate::Slate;
pub use crate::natural::N;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unkown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
