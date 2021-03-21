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

    /// The type of Block stored in the Chunk
    type Block;

    /// The type used for storage of Block in the Chunk
    type BlockStorage:
        AsRef<[Self::Block]>
        + AsMut<[Self::Block]>
        + Default;
}
