//! Natural numbers
use crate::simd::MachineWord;

/// A natural number, including zero
pub struct N<'n> {
    words: Vec<&'n MachineWord>,
    precision: usize,
}
