use std::ptr::NonNull;

use super::{HeapStack, Region};

pub struct BumpAllocator<T: Default> {
    next: NonNull<T>,
    last: NonNull<T>,
    stack: HeapStack<Region<T>>,
}

impl<T: Default> BumpAllocator<T> {
    pub fn new() -> Self {
        BumpAllocator {
            next: NonNull::dangling(),
            last: NonNull::dangling(),
            stack: HeapStack::new(),
        }
    }

    pub fn alloc(&mut self) -> Region<T> {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn allocate() {
        let mut bump: BumpAllocator<u64> = BumpAllocator::new();
        let first = bump.alloc();
        let second = bump.alloc();
    }
}