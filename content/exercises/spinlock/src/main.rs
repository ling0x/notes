use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
};

/// A Spinlock is the simplest possible implementation of a mutex, its general form looks like this
/// ```rust
/// static LOCKED: AtomicBool = AtomicBool::new(false);
/// // 1. To grab a lock, we repeatedly execute compareandswap until it succeeds.
/// // The CPU “spins” in this very short loop.
/// while LOCKED.compare_and_swap(false, true, Ordering::Acquire) {
///   // 4. Spinning is wasteful, so we use an intrinsic to instruct the CPU to
///   // enter a low-power mode.
///   std::sync::atomic::spin_loop_hint();
/// }
/// // 2. Only one thread at a time can be here.
/// /* Critical section  */
/// // 3. To release the lock, we do a single atomic store.
/// LOCKED.store(false, Ordering::Release);
/// ```
/// Checkout the "Spinlock Considered Harmful" post:
/// https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
///
/// All we need is a single boolean that indicates whether it is locked or not.
pub struct SpinLock<T> {
    /// QUESTION: What does an atomic bool does conceptually?
    /// - Stores a true/false value that can be shared across threads without a mutex.
    /// - Provides atomic operations like load, store, swap, compare_exchange,
    ///   and bitwise ops (fetch_or, fetch_and, fetch_not), each taking a memory
    ///   Ordering to control how operations are seen across threads.
    /// - Uses CPU atomic instructions so an update is either fully seen or not seen at all by other threads; there is no partial write.
    locked: AtomicBool,

    /// We need to have an exclusive reference (&mut T) to the data protected by the lock
    /// The value field holds the generic over the type of data the lock protects
    /// We use UnsafeCell for interior mutability
    value: UnsafeCell<T>,
}

/// A Safe Interface Using a Lock Guard
///
/// Wrap the reference in a type that implements the Drop trait to do something
/// when it is dropped.
///
/// The existence of a Guard means that the SpinLock has been locked.
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

/// In order to make the UnsafeCell to be shareable between threads, we need to
/// promise to the compiler that it is actually safe for our type to be shared
/// between threads.
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

/// Mutual Exclusion — the guarantee that only one thread can access the
/// protected data at any given moment.
///
/// Spinlock Mechanism
impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// The lock method returns a Guard, such that the user isn't required to
    /// write unsafe, unchecked code when using the lock to protect their data
    pub fn lock(&self) -> Guard<'_, T> {
        while self.locked.swap(true, Acquire) {
            // Within the while loop, we use a spin loop hint, which emits a
            // special CPU instruction that says “I’m in a tight busy‑wait loop;
            // expect lots of repeated reads and no useful work.” This lets
            // the core change how it treats that thread without involving
            // the OS scheduler.
            spin_loop();
        }
        Guard { lock: self }
    }

    /// We use acquire and release memory ordering to make sure that every
    /// unlock() call establishes a happens-before relationship with the
    /// lock() calls that follow.
    ///
    /// # Safety
    ///
    /// The &mut T from lock() must be gone!
    /// (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }

    /// use a compare-and-exchange operation to atomically check if the boolean
    /// is false and set it to true if that’s the case
    pub fn cas(&self) {
        while self
            .locked
            .compare_exchange_weak(false, true, Acquire, Relaxed)
            .is_err()
        {
            spin_loop();
        }
    }
}

/// To make Guard<T> behave like an (exclusive) reference, we have to implement
/// the special Deref and DerefMut traits
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // # Safety:
        // The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // # Safety:
        // The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

/// Add our own implementation of Send and Sync with the right bounds to make sure
/// our Guard is only Sync if T is Sync (and Send if T is Send)
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

/// Implement Drop for Guard, allowing us to the unsafe unlock method safe again
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
    println!("{:#?}", g.as_slice());
}
