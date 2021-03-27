//! Pure Rust implementation of exact arithmetic and algorithms for computer geometry applications.

pub mod number;
pub mod memory;
pub mod simd;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("allocation would exceed region maximmum of {max:?} chunks")]
    RegionLimit {
        max: usize,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
