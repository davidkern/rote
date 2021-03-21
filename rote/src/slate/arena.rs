//! A heap-based stack which owns its entries.
//! This is based on the code for std::collections::LinkedList,
//! but implements a singly-linked list.

use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Policy;

/// A heap-based stack which owns its entries
pub struct HeapStack<T> {
    top: Option<NonNull<Entry<T>>>,
    len: usize,
    marker: PhantomData<Box<Entry<T>>>,
}

struct Entry<T> {
    next: Option<NonNull<Entry<T>>>,
    val: T,
}

impl<T> Entry<T> {
    fn new(val: T) -> Self {
        Entry { next: None, val }
    }

    fn into_owned(self: Box<Self>) -> T {
        self.val
    }
}

impl<T> HeapStack<T> {
    /// Pushes entry onto the stack
    #[inline]
    fn push_entry(&mut self, mut entry: Box<Entry<T>>) {
        entry.next = self.top;

        let entry = Some(Box::leak(entry).into());

        self.top = entry;
        self.len += 1;
    }

    /// Pops the entry from the stack
    #[inline]
    fn pop_entry(&mut self) -> Option<Box<Entry<T>>> {
        self.top.map(|entry| {
            let entry = unsafe { Box::from_raw(entry.as_ptr()) };
            self.top = entry.next;
            self.len -= 1;

            entry
        })
    }
}

impl<T> Default for HeapStack<T> {
    /// Creates an empty `HeapStack<T>`
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HeapStack<T> {
    /// Creates an empty `HeapStack<T>`
    #[inline]
    pub const fn new() -> Self {
        HeapStack { top: None, len: 0, marker: PhantomData }
    }
    
    /// Returns a reference to the top entry, or `None` if the stack is empty
    #[inline]
    pub fn top(&self) -> Option<&T> {
        unsafe { self.top.as_ref().map(|entry| &entry.as_ref().val ) }
    }

    /// Returns a mutable reference to the top entry, or `None if the stack
    /// is empty
    #[inline]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        unsafe { self.top.as_mut().map(|entry| &mut entry.as_mut().val ) }
    }

    /// Pushes entry onto the stack
    #[inline]
    pub fn push(&mut self, val: T) {
        self.push_entry(Box::new(Entry::new(val)));
    }

    /// Removes the top entry and returns it, or `None` if the stack is empty
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.pop_entry().map(Entry::into_owned)
    }
}
