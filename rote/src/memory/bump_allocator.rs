use std::marker::PhantomData;
use std::ptr::NonNull;

use super::{HeapStack, Policy, Region};

pub struct BumpAllocator<P: Policy, T: Default> {
    next: NonNull<T>,
    last: NonNull<T>,
    stack: HeapStack<Region<P, T>>,
}

impl<P: Policy, T: Default> BumpAllocator<P, T> {
    pub fn new() -> Self {
        BumpAllocator {
            next: NonNull::dangling(),
            last: NonNull::dangling(),
            stack: HeapStack::new(),
        }
    }
}
