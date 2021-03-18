use std::marker::PhantomData;

pub struct Contiguous<'slate, T> {
    _items: T,
    _phantom: PhantomData<&'slate T>,
}
