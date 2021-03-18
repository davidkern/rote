/// A Symbol on the Slate
pub struct Symbol<'slate, T>
where
    T: Copy
{
    pub(crate) value: &'slate mut T,
}

impl<'slate, T> Symbol<'slate, T>
where
    T: Copy
{
    /// Get a copy of the Symbol value
    pub fn value(&self) -> T {
        *self.value
    }
}
