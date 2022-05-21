use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct ID(pub u64);

impl ID {
    /// Creates new unique ID starting from 0
    /// ```
    /// # use rayimg::ID;
    /// let (id1, id2) = (ID::new(), ID::new());
    /// assert!(id1 == ID(0) && id2 == ID(1));
    /// ```
    pub fn new() -> Self {
        Self::next_id()
    }

    fn next_id() -> Self {
        static CURRENT_ID: AtomicU64 = AtomicU64::new(0);
        ID(CURRENT_ID.fetch_add(1, Ordering::Relaxed))
    }
}