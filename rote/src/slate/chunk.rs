use std::marker::PhantomData;
use super::Policy;

// TODO: deallocate on Drop

pub struct Chunk<'slate, TPolicy>
where
    TPolicy: 'slate + Policy,
{
    pub policy: PhantomData<TPolicy>,
    pub next: Option<&'slate Chunk<'slate, TPolicy>>,
    pub blocks: &'slate [TPolicy::Block],
}

impl<'slate, TPolicy> Chunk<'slate, TPolicy>
where
    TPolicy: 'slate + Policy,
{
    pub fn new() -> Self {
        let blocks: &'slate [TPolicy::Block];

        // Use layout of the block for size and alignment
        let layout = std::alloc::Layout::array::<TPolicy::Block>(TPolicy::BLOCKS_PER_CHUNK).unwrap();
        debug_assert!(layout.size() > 0);

        unsafe {
            // Allocate
            let blocks_ptr = std::alloc::alloc(layout) as *mut TPolicy::Block;

            // Initialize
            for i in 0..TPolicy::BLOCKS_PER_CHUNK {
                blocks_ptr.offset(i as isize).write(TPolicy::Block::default());
            }

            // Turn into a slice
            blocks = std::slice::from_raw_parts(blocks_ptr, TPolicy::BLOCKS_PER_CHUNK); 
        }

        Self {
            policy: PhantomData,
            blocks: blocks,
            next: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::slate::ChunkLimit;
    use crate::slate::block::{Block128, Block256, Block512};

    struct Policy128;
    struct Policy256;
    struct Policy512;

    impl Policy for Policy128 {
        const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        const BLOCKS_PER_CHUNK: usize = 1;
        
        type Block = Block128<[u64; 2]>;
    }

    impl Policy for Policy256 {
        const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        const BLOCKS_PER_CHUNK: usize = 1;
        
        type Block = Block256<[u64; 2]>;
    }

    impl Policy for Policy512 {
        const CHUNK_LIMIT: ChunkLimit = ChunkLimit::NoLimit;
        const BLOCKS_PER_CHUNK: usize = 1;
        
        type Block = Block512<[u64; 2]>;
    }

    #[test]
    fn size_and_alignment() {
        let chunk128: Chunk<'_, Policy128> = Chunk::new();

        assert_eq!(Policy128::BLOCKS_PER_CHUNK, chunk128.blocks.len());
        assert_eq!(16, std::mem::align_of_val(&chunk128.blocks[0]));
        assert_eq!(16, std::mem::size_of_val(&chunk128.blocks[0]));

        let chunk256: Chunk<'_, Policy256> = Chunk::new();

        assert_eq!(Policy256::BLOCKS_PER_CHUNK, chunk256.blocks.len());
        assert_eq!(32, std::mem::align_of_val(&chunk256.blocks[0]));
        assert_eq!(32, std::mem::size_of_val(&chunk256.blocks[0]));

        let chunk512: Chunk<'_, Policy512> = Chunk::new();

        assert_eq!(Policy512::BLOCKS_PER_CHUNK, chunk512.blocks.len());
        assert_eq!(64, std::mem::align_of_val(&chunk512.blocks[0]));
        assert_eq!(64, std::mem::size_of_val(&chunk512.blocks[0]));
    }
}
