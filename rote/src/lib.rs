//! Pure Rust implementation of exact arithmetic and algorithms for computer geometry applications.

pub mod expression;
pub mod natural;
pub mod memory;

pub use rote_macros::rote;

//pub use crate::natural::N;

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

#[cfg(test)]
mod test {
    use crate::rote;

    #[test]
    fn expression() {
        let x = 0;
        let _expr = rote!(1 + x);        
    }
}