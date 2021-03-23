use std::marker::PhantomData;
use std::alloc;
use std::ptr;

pub struct Region<T: Default> {
    marker: PhantomData<[T]>,
    start: *mut T,
    end: *mut T,
}

impl<T: Default> Region<T> {
    // TODO: allocation is falible
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
                
                // TODO: Initialize memory
    
                Region {
                    marker: PhantomData,
                    start, end,
                }
            }    
        }
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