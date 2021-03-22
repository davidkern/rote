//! Memory-management for arbitrary-precision values.
//! 
//! Analogous to the "slates" historically used by school-children to practice arithmetic.

use std::marker::PhantomData;
use std::cell::UnsafeCell;
use std::slice::IterMut;
use crate::{Error, Result};
use crate::memory::{Policy, ChunkLimit};

pub struct Slate<'slate, TPolicy: 'slate + Policy> {
    _policy: PhantomData<&'slate TPolicy>,
    inner: UnsafeCell<SlateInner<TPolicy>>,
}

impl<'slate, TPolicy: 'slate + Policy> Slate<'slate, TPolicy> {
    /// Construct a new `Slate`
    pub fn new() -> Self {
        let slate = Self {
            _policy: PhantomData,
            inner: UnsafeCell::new(SlateInner::new()),
        };

        let inner = unsafe { slate.inner.get().as_mut().unwrap() };
        inner.grow();

        slate
    }

    pub fn block(&'slate self) -> Option<&'slate mut TPolicy::Block> {
        let inner = unsafe { self.inner.get().as_mut().unwrap() };

        inner.next_block()
    }
}

struct SlateInner<TPolicy: Policy> {
    _policy: PhantomData<TPolicy>,
    chunk_count: usize,
    block_index: usize,
    chunk: Option<Box<Chunk<TPolicy>>>,
}

impl<TPolicy: Policy> SlateInner<TPolicy> {
    fn new() -> Self {
        Self {
            _policy: PhantomData,
            chunk_count: 0,
            block_index: 0,
            chunk: None,
        }
    }

    /// Grows the slate storage by an additional heap-allocated chunk
    fn grow(&mut self) {
        // create a new chunk
        // let new_chunk = Some(Chunk::new_boxed());
        
        // replace old with new chunk
        // let old_chunk = std::mem::replace(&mut self.chunk, new_chunk);

        // // link the old chunk to the new chunk
        // let mut chunk = self.chunk.as_mut().unwrap();
        // chunk.next = old_chunk;

        // // keep track of accounting
        // self.block_index = 0;
        // self.chunk_count += 1;
    }

    /// Allocates a block
    fn next_block(&mut self) -> Option<&mut TPolicy::Block> {
        // let blocks = self.chunk.as_mut().unwrap().blocks.as_mut();
        // if self.block_index == blocks.len() {
        //     self.grow();
        // }

        // let block: *mut TPolicy::Block = &mut blocks[self.block_index];
        // unsafe { block.as_mut() }
        unimplemented!();
    }
}

struct Chunk<TPolicy: Policy> {
    pub next: Option<Box<Chunk<TPolicy>>>,
    pub blocks: TPolicy::BlockStorage,
}

impl<TPolicy: Policy> Chunk<TPolicy> {
    fn new_boxed() -> Box<Self> {
        let chunk = Self {
            next: None,
            blocks: TPolicy::BlockStorage::default(),
        };

        Box::new(chunk)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestPolicy;

    impl Policy for TestPolicy {
        const CHUNK_LIMIT: ChunkLimit = ChunkLimit::Limit { limit: 2 };
        type Block = u64;
        type BlockStorage = [u64; 2];
    }

    // #[test]
    // fn concurrent_mutable_blocks() {
    //     let slate = Slate::<TestPolicy>::new();

    //     let b0 = slate.block().unwrap();
    //     let b1 = slate.block().unwrap();
    //     let b2 = slate.block().unwrap();

    //     *b0 = 1234;
    //     *b1 = 4321;
    // }
}