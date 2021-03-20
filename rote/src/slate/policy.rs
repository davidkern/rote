/// Limits on allocating chunks
pub enum ChunkLimit {
    NoLimit,
    Limit {
        limit: usize,
    },
}

pub trait Policy {
    /// Maximum number of chunks allowed to allocated from
    /// the global allocator. ChunkLimit::Limit(1) will allocate one
    /// chunk when the Slate is constructed and will perform no other
    /// allocations. ChunkLimit::NoLimit will not limit the number of
    /// allocations.
    const CHUNK_LIMIT: ChunkLimit;

    /// The number of Blocks in each Chunk
    const BLOCKS_PER_CHUNK: usize;

    /// The type of Block stored in the Chunk
    type Block: Copy + Default;
}
