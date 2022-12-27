// My implementation of a Counter data structure that efficiently keeps track
// of the uniqueness of the elements. All operations run in at most amortized
// constant time.

// Used in day 6 to create an efficient sliding window that maintains uniqueness.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct UniqCounter<T> {
    items: HashMap<T, i32>,
    dups: HashSet<T>,
}

impl<T: Eq + Hash + Copy> UniqCounter<T> {
    pub fn new(capacity: usize) -> Self {
        UniqCounter {
            items: HashMap::with_capacity(capacity),
            dups: HashSet::with_capacity(capacity),
        }
    }

    pub fn is_unique(&self) -> bool {
        self.dups.is_empty()
    }

    pub fn add(&mut self, v: T) {
        if self.items.contains_key(&v) {
            self.dups.insert(v);
        }
        *self.items.entry(v).or_default() += 1
    }

    pub fn remove(&mut self, v: T) {
        if let Some(&n) = self.items.get(&v) {
            if n == 1 {
                self.items.remove(&v);
                return;
            }
            if n == 2 {
                self.dups.remove(&v);
            }
            self.items.insert(v, n - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UniqCounter;

    #[test]
    fn test_unique_counter() {
        let mut uc = UniqCounter::new(15);

        uc.add('a');
        assert!(uc.is_unique());

        uc.add('a');
        assert!(!uc.is_unique());

        uc.add('c');
        assert!(!uc.is_unique());

        uc.remove('a');
        assert!(uc.is_unique());

        uc.remove('a');
        assert!(uc.is_unique());

        uc.remove('c');
        assert!(uc.is_unique());

        uc.add('a');
        uc.add('b');
        uc.add('c');
        uc.add('d');
        uc.add('e');
        uc.add('f');
        assert!(uc.is_unique());
        uc.add('a');
        assert!(!uc.is_unique());
        uc.add('a');
        uc.add('a');
        uc.remove('a');
        uc.remove('a');
        uc.remove('a');
        assert!(uc.is_unique());
    }
}
