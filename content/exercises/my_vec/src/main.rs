// This project implements Vec from scratch (https://doc.rust-lang.org/nomicon/vec/vec.html)

use std::ptr::NonNull;

/// Layout:
/// A Vec has three parts: a pointer to the allocation, the size of the allocation,
/// and the number of elements that have been initialized.
pub struct Vec<T> {
    // NonNull is a wrapper around a raw pointer, which is covariant over T
    // and is decalred to never be null.
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

// Vec<T> is Send/Sync if T is Send/Sync
// (this produces the same results as using Unique<T>)
unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

fn main() {
    println!("Hello, world!");
}
