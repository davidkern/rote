//! Memory-management for arbitrary-precision values.
//! 
//! Analogous to the "slates" historically used by school-children to practice arithmetic.

use std::marker::PhantomData;
use super::Chunk;
use super::Policy;


pub struct Slate<'slate, TPolicy: Policy> {
    _policy: PhantomData<TPolicy>,
    _chunk: Chunk<'slate, TPolicy>,
}

impl<'slate, TPolicy: Policy> Slate<'slate, TPolicy> {
    // /// Construct a new `Slate`
    // pub fn new() -> Self {
    //     Self {
    //         chunk: ,
    //     }
    // }
}
