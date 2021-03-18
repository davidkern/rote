//! Memory-management for arbitrary-precision values.
//! 
//! Analogous to the "slates" historically used by school-children to practice arithmetic.
use std::marker::PhantomData;

/// A `Slate` holds contiguous Symbol<T>.
pub struct Slate<T> {
    bump: bumpalo::Bump,
    _phantom: PhantomData<T>,
}

impl<T> Slate<T> {
    /// Construct a new `Slate`
    pub fn new() -> Self {
        Self {
            bump: bumpalo::Bump::new(),
            _phantom: PhantomData,
        }
    }

    /// Erases all Symbol<T>
    pub fn erase(&mut self) {
        self.bump.reset();
    }
}

