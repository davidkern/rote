use super::Symbol;

/// A Place where a Symbol may be, but has not been, written
pub struct Place<'slate, T>
where
    T: Copy
{
    place: &'slate mut T,
}

impl<'slate, T> Place<'slate, T>
where
    T: Copy
{
    /// Writes a Symbol<T> into a Place, returning the newly written Symbol<T>
    pub fn write(self, value: T) -> Symbol<'slate, T> {
        *self.place = value;

        Symbol {
            value: self.place,
        }
    }
}
