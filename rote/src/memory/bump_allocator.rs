use std::marker::PhantomData;
use std::ptr::NonNull;

use super::{HeapStack, Policy};

pub struct BumpAllocator<TPolicy: Policy> {
    next: NonNull<TPolicy::Block>,
    last: NonNull<TPolicy::Block>,
    stack: HeapStack<TPolicy::BlockStorage>,
}

impl<TPolicy: Policy> BumpAllocator<TPolicy> {
    pub fn new() -> Self {
        BumpAllocator {
            next: NonNull::dangling(),
            last: NonNull::dangling(),
            stack: HeapStack::new(),
        }
    }
}
