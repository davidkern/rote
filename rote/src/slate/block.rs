pub trait Block { }

/// 128-bit block
#[repr(align(16))]
#[derive(Copy, Clone, Default)]
pub struct Block128 {
    pub words: [u64; 2],
}

/// 256-bit block
#[repr(align(32))]
#[derive(Copy, Clone, Default)]
pub struct Block256 {
    pub words: [u64; 4],
}

/// 512-bit block
#[repr(align(64))]
#[derive(Copy, Clone, Default)]
pub struct Block512 {
    pub words: [u64; 8],
}

impl Block for Block128 { }
impl Block for Block256 { }
impl Block for Block512 { }
