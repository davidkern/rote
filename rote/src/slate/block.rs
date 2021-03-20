pub trait Block { }

/// 128-bit block
#[repr(align(16))]
#[derive(Copy, Clone, Default)]
pub struct Block128<T: ?Sized + Copy + Default> {
    pub words: T
}

/// 256-bit block
#[repr(align(32))]
#[derive(Copy, Clone, Default)]
pub struct Block256<T: ?Sized + Copy + Default> {
    pub words: T
}

/// 512-bit block
#[repr(align(64))]
#[derive(Copy, Clone, Default)]
pub struct Block512<T: ?Sized + Copy + Default> {
    pub words: T
}

impl<T: Copy + Default> Block for Block128<T> { }
impl<T: Copy + Default> Block for Block256<T> { }
impl<T: Copy + Default> Block for Block512<T> { }
