// use std::marker::PhantomData;
// use super::Policy;

// /// Allocate blocks on the heap, a chunk of blocks at a time
// pub struct Chunk<TPolicy>
// where
//     TPolicy: Policy
// {
//     _policy: PhantomData<TPolicy>,
//     inner: Box<ChunkInner<TPolicy>>,
// }

// impl<TPolicy> Chunk<TPolicy>
// where
//     TPolicy: Policy,
// {
//     /// Create a new chunk on the heap
//     pub fn new_boxed() -> Self {
//         Self {
//             _policy: PhantomData,
//             inner: ChunkInner::boxed_new(None),
//         }
//     }

//     /// Increases the capacity by one heap-allocated chunk
//     pub fn grow(&mut self) {
//         // Create a new chunk
//         let mut new_inner = ChunkInner::boxed_new(None);

//         // Replace the old chunk with the new chunk
//         let old_inner = std::mem::replace(&mut self.inner, new_inner);

//         // Link the old chunk to the new chunk
//         self.inner.next = Some(old_inner);
//     }

//     // /// The number of blocks in this chunk
//     // pub fn block_count(&self) -> usize {
//     //     self.inner.as_ref().blocks.as_ref().len()
//     // }
// }

// struct ChunkInner<TPolicy>
// where
//     TPolicy: Policy
// {
//     pub next: Option<Box<ChunkInner<TPolicy>>>,
//     pub blocks: TPolicy::BlockStorage,
// }

// impl<TPolicy> ChunkInner<TPolicy>
// where
//     TPolicy: Policy
// {
//     fn boxed_new(next: Option<Box<Self>>) -> Box<Self> {
//         Box::new (
//             ChunkInner {
//                 next,
//                 blocks: TPolicy::BlockStorage::default(),
//             }
//         )
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::slate::ChunkLimit;
//     use crate::slate::block::{Block128, Block256, Block512};

//     struct Policy128;
//     struct Policy256;
//     struct Policy512;

//     impl Policy for Policy128 {
//         const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        
//         type Block = Block128;
//         type BlockStorage = [Self::Block; 1];
//     }

//     impl Policy for Policy256 {
//         const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        
//         type Block = Block256;
//         type BlockStorage = [Self::Block; 1];
//     }

//     impl Policy for Policy512 {
//         const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        
//         type Block = Block512;
//         type BlockStorage = [Self::Block; 1];
//     }

//     #[test]
//     fn size_and_alignment() {
//         // let chunk128: Chunk<Policy128> = Chunk::new();

//         // assert_eq!(16, std::mem::align_of_val(&chunk128[0]));
//         // assert_eq!(16, std::mem::size_of_val(&chunk128[0]));

//         // let chunk256: Chunk<Policy256> = Chunk::new();

//         // assert_eq!(32, std::mem::align_of_val(&chunk256[0]));
//         // assert_eq!(32, std::mem::size_of_val(&chunk256[0]));

//         // let chunk512: Chunk<Policy512> = Chunk::new();

//         // assert_eq!(64, std::mem::align_of_val(&chunk512[0]));
//         // assert_eq!(64, std::mem::size_of_val(&chunk512[0]));
//     }
// }
