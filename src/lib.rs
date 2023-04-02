#![no_std]

//! RuntimeID provides lightweight unique identifers per 'run' of a program.
//!
//! Internally this is just a usize that counts up from zero using atomic instructions. This makes RuntimeIDs
//! extremely cheap to create and compare with the downside that they cannot be serialized.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::hash::{Hash, Hasher};

static ID: AtomicUsize = AtomicUsize::new(0);

/// Opaque ID that's unique per 'run' of a program.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RuntimeID(usize);

impl RuntimeID {
    /// Creates a new unique RuntimeID.
    ///
    /// # Example
    /// ```
    /// let a = runtime_id::RuntimeID::new();
    /// let b = runtime_id::RuntimeID::new();
    /// assert_ne!(a, b);
    /// ```
    #[inline]
    pub fn new() -> Self {
        RuntimeID(ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl PartialEq for RuntimeID {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for RuntimeID {}

impl Hash for RuntimeID {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_ne_bytes());
    }
}

#[cfg(test)]
mod test {
    use crate::RuntimeID;
    use ahash::AHasher;
    use core::hash::{Hash, Hasher};

    #[test]
    fn basic() {
        let id_0 = RuntimeID::new();
        let id_0_copy = id_0;
        let id_1 = RuntimeID::new();

        assert_ne!(id_0, id_1);
        assert_eq!(id_0, id_0);
        assert_eq!(id_0, id_0_copy);
    }

    #[test]
    fn hash() {
        let mut hasher = AHasher::default();
        RuntimeID::new().hash(&mut hasher);
        let test_hash = hasher.finish();

        const ITTERATIONS: usize = 100000; 
        for _ in 0..ITTERATIONS {
            let mut hasher = AHasher::default();
            RuntimeID::new().hash(&mut hasher);
            let hash = hasher.finish();
            assert_ne!(hash, test_hash);
        }
    }
}