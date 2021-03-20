//! Memory-management for arbitrary-precision values.
//! 
//! Analogous to the "slates" historically used by school-children to practice arithmetic.

use std::marker::PhantomData;
use crate::{Error, Result};
use super::{Chunk, ChunkLimit, Policy};


pub struct Slate<'slate, TPolicy: Policy> {
    _policy: PhantomData<TPolicy>,
    chunk: Box<Chunk<'slate, TPolicy>>,
    chunk_count: usize,
    free_block_index: usize,
}

impl<'slate, TPolicy: Policy> Slate<'slate, TPolicy> {
    /// Construct a new `Slate`
    pub fn new() -> Self {
        Self {
            _policy: PhantomData,
            chunk: Box::new(Chunk::new()),
            chunk_count: 1,
            free_block_index: 0,
        }
    }

    pub fn block(&'slate mut self) -> Result<&mut TPolicy::Block> {
        // allocate a new chunk if no blocks are free
        if self.free_block_index == self.chunk.blocks.len() {
            // do not allocate if it would exceed chunk limit
            if let ChunkLimit::Limit{ limit } = TPolicy::CHUNK_LIMIT {
                if limit == self.chunk_count { return Err(Error::ChunkLimit { limit }) }
            }

            let new_chunk = Box::new(Chunk::new());
            let old_chunk = std::mem::replace(&mut self.chunk, new_chunk);
            self.chunk.as_mut().next = Some(old_chunk);

            self.chunk_count += 1;
            self.free_block_index = 0;
        }

        // get next free block and increment the free block index
        let block = &mut self.chunk.blocks[self.free_block_index];
        self.free_block_index += 1;
        
        Ok(block)
    }
}

#[cfg(test)]
mod test {

}