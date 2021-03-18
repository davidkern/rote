// use std::marker::PhantomData;
// use crate::Result;
// use super::{Contiguous, Place, Symbol};


// pub struct Storage<'storage, T> {
//     bump: bumpalo::Bump,
//     _phantom: PhantomData<&'storage T>,
// }

// impl<'storage, T> Storage<'storage, T> {
//     /// Construct a new `Slate`
//     pub fn new() -> Self {
//         Self {
//             bump: bumpalo::Bump::new(),
//             _phantom: PhantomData,
//         }
//     }

//     pub fn write<Writer>(writer: Writer) -> Result<Contiguous<Symbol<'storage, T>>>
//     where
//         Writer: Fn(Contiguous<Place<'storage, T>>) -> Result<Contiguous<Symbol<'storage, T>>>
//     {
//         unimplemented!();
//     }
// }