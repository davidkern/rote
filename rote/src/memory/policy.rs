pub trait MemoryPolicy {
    const POLICY: Policy;
}

pub enum Policy {
    Fixed { len: usize },
    Dynamic,
}

impl Policy {
    pub const fn initial(self) -> usize {
        match self {
            Self::Fixed { len } => len,
            Self::Dynamic => 0,
        }
    }
}
