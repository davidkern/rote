use std::marker::PhantomData;
use std::ptr::NonNull;

use super::{HeapStack, MemoryPolicy, Policy, Region};

pub struct BumpAllocator<P: MemoryPolicy, T: Default> {
    pub policy: PhantomData<*const P>,
    next: NonNull<T>,
    last: NonNull<T>,
    stack: HeapStack<Region<T>>,
}

impl<P: MemoryPolicy, T: Default> BumpAllocator<P, T> {
    pub fn new() -> Self {
        BumpAllocator {
            policy: PhantomData,
            next: NonNull::dangling(),
            last: NonNull::dangling(),
            stack: HeapStack::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn allocate() {
        struct FixedPolicy;
        impl MemoryPolicy for FixedPolicy {
            const POLICY: Policy = Policy::Fixed { len: 1 };
        }

        let mut bump: BumpAllocator<FixedPolicy, u64> = BumpAllocator::new();
        // let first = bump.alloc();
        // let second = bump.alloc();
    }
}