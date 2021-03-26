use std::marker::PhantomData;
use std::alloc;
use std::ptr;

pub struct Region<T: Default> {
    marker: PhantomData<[T]>,
    start: *mut T,
    end: *mut T,
}

impl<T: Default> Region<T> {
    /// Construct a new Region.
    /// panics if allocation fails
    pub fn with_capacity(len: usize) -> Self {
        if len == 0 {
            // Zero-sized regions are not allocated
            Region {
                marker: PhantomData,
                start: ptr::null_mut(),
                end: ptr::null_mut(),
            }
        } else {
            // Allocate non-zero-sized region
            let layout = alloc::Layout::array::<T>(len).unwrap();
            unsafe {
                let start = alloc::alloc(layout) as *mut T;
                let end = start.offset(len as isize);
                
                let mut region = Region {
                    marker: PhantomData,
                    start, end,
                };

                region.clear();

                region
            }    
        }
    }

    /// Resets all region contents to T::default()
    pub fn clear(&mut self) {
        let mut idx = self.start;
        while idx != self.end {
            unsafe {
                *idx = T::default();
                idx = idx.offset(1);
            }
        }

    }

    /// Gets start pointer
    pub fn start(&mut self) -> *mut T {
        self.start
    }

    /// Gets end pointer
    pub fn end(&mut self) -> *mut T {
        self.end
    }
}

impl<T: Default> Drop for Region<T> {
    fn drop(&mut self) {
        if ! self.start.is_null() {
            unsafe {
                let len = self.end.offset_from(self.start) as usize;
                let layout = alloc::Layout::array::<T>(len).unwrap();
                alloc::dealloc(self.start as *mut u8, layout);    
            }    
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_and_drop() {
        let _region = Region::<u64>::with_capacity(1_000_000);
    }

    #[test]
    fn zero_sized() {
        let _region = Region::<u64>::with_capacity(0);
    }
}