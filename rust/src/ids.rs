use std::hash::Hash;
use std::sync::atomic::AtomicU64;

pub trait Id: Clone + Ord + PartialOrd + PartialEq + Eq + Hash {
    fn new(value: u64) -> Self;
}

pub struct IdGenerator {
    next_id: AtomicU64,
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator {
            next_id: AtomicU64::new(1),
        }
    }

    pub fn generate<T: Id>(&mut self) -> T {
        let next_id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return T::new(next_id);
    }
}
