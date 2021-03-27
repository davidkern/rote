//! Natural numbers
use crate::simd::MachineWord;
use crate::memory::Slice;

/// A natural number, including zero
#[derive(Clone, Debug)]
pub struct N<'n> {
    /// The data representing this number
    /// least-significant word first
    words: Slice<'n, MachineWord>,
}
