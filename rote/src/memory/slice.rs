/// A slice of items, which need not be contiguous
#[derive(Clone, Debug)]
pub struct Slice<'slice, T> {
    inner: SliceInner<'slice, T>,
}

#[derive(Clone, Debug)]
enum SliceInner<'slice, T> {
    Contiguous(&'slice [T]),
    Segmented(Vec<&'slice [T]>),
}

/// Creates a Slice from a primitive slice
impl<'slice, T> From<&'slice [T]> for Slice<'slice, T> {
    fn from(other: &'slice [T]) -> Self {
        Self {
            inner: SliceInner::Contiguous(other)
        }
    }
}

/// Creates a Slice from a slice of primitive slices
impl<'slice, T> From<&[&'slice [T]]> for Slice<'slice, T> {
    fn from(slices: &[&'slice [T]]) -> Self {
        Self {
            inner: SliceInner::Segmented(Vec::from(slices))
        }
    }
}

impl<'slice, T> Slice<'slice, T> {
    /// Creates a new empty slice
    pub fn new() -> Self {
        Self {
            inner: SliceInner::Contiguous(&[])
        }
    }

    /// Returns true if the slice is contiguous
    pub fn is_contiguous(&self) -> bool {
        match self.inner {
            SliceInner::Contiguous(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construction() {
        let arr1 = [0, 1, 2];
        let arr2 = [3, 4, 5];

        let segments = [&arr1[..], &arr2[..]];

        let contiguous = Slice::from(&arr1[..]);
        assert_eq!(true, contiguous.is_contiguous());

        let segmented: Slice<'_, i32> = Slice::from(&segments[..]); 
        assert_eq!(false, segmented.is_contiguous());
    }
}
