use std::marker::PhantomData;
use super::Policy;

pub struct Region<P, T: Default> {
    pub policy: PhantomData<*const P>,
    items: T,
}

impl<P: Policy, T: Default> Region<P, T> {
    pub fn new(len: usize) -> Self {
        Self {
            policy: PhantomData,
            items: Default::default(),
        }
    }
}
