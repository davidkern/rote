use std::marker::PhantomData;
use std::cell::UnsafeCell;

use super::{HeapStack, Region};

pub struct BumpAllocator<'alloc, T: Default> {
    marker: PhantomData<&'alloc T>,
    inner: UnsafeCell<BumpAllocatorInner<T>>,
}

struct BumpAllocatorInner<T: Default> {
    item_count: usize,
    items_per_region: usize,
    next: *mut T,
    end: *mut T,
    stack: HeapStack<Region<T>>,
}

impl<'alloc, T: Default> BumpAllocator<'alloc, T> {
    pub fn new(items_per_region: usize) -> Self {
        Self {
            marker: PhantomData,
            inner: UnsafeCell::new(
                BumpAllocatorInner {
                    item_count: 0,
                    items_per_region,
                    next: std::ptr::null_mut(),
                    end: std::ptr::null_mut(),
                    stack: HeapStack::new(),        
                }
            )
        }
    }

    pub fn alloc(&'alloc self) -> &'alloc mut T {
        let inner = unsafe { &mut *self.inner.get() };

        // allocate another region if necessary
        if inner.next == inner.end {
            let mut region = Region::with_capacity(inner.items_per_region); 

            inner.next = region.start();
            inner.end = region.end();
    
            inner.stack.push(region);
        }

        let allocation = inner.next;
        unsafe {
            inner.next = inner.next.offset(1);
            inner.item_count += 1;

            &mut *allocation
        }
    }

    pub fn region_count(&'alloc self) -> usize {
        let inner = unsafe{ &mut *self.inner.get() };

        inner.stack.len()
    }

    pub fn item_count(&'alloc self) -> usize {
        let inner = unsafe{ &mut *self.inner.get() };

        inner.item_count
    }

    pub fn items_per_region(&'alloc self) -> usize {
        let inner = unsafe { &mut *self.inner.get() };

        inner.items_per_region
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn allocate() {
        let bump: BumpAllocator<u64> = BumpAllocator::new(2);
        assert_eq!(0, bump.item_count());
        assert_eq!(0, bump.region_count());

        let first = bump.alloc();
        assert_eq!(1, bump.item_count());
        assert_eq!(1, bump.region_count());

        let second = bump.alloc();
        assert_eq!(2, bump.item_count());
        assert_eq!(1, bump.region_count());

        *first = 1;
        *second = 2;

        assert_ne!(first, second);
        assert_ne!(*first, *second);
    }

    #[test]
    pub fn allocate_large() {
        let bump: BumpAllocator<u64> = BumpAllocator::new(1_000_000);
        assert_eq!(0, bump.item_count());
        assert_eq!(0, bump.region_count());

        for _ in 0..10_000_000 {
            bump.alloc();
        }

        assert_eq!(10_000_000, bump.item_count());
        assert_eq!(10, bump.region_count());
    }
}