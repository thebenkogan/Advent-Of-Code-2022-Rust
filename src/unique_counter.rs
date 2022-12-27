// My implementation of a Counter data structure that efficiently keeps track
// of the uniqueness of the elements. All operations run in at most amortized
// constant time.

// Used in day 6 to create an efficient sliding window that maintains uniqueness.

use std::collections::HashMap;
use std::hash::Hash;

pub struct UniqCounter<T> {
    items: HashMap<T, i32>, // entry -> count of entry
    dups: i32,              // number of duplicate entries in the counter
}

impl<T: Eq + Hash + Copy> UniqCounter<T> {
    pub fn new(capacity: usize) -> Self {
        UniqCounter {
            items: HashMap::with_capacity(capacity),
            dups: 0,
        }
    }

    pub fn is_unique(&self) -> bool {
        self.dups == 0
    }

    pub fn add(&mut self, v: T) {
        if self.items.contains_key(&v) {
            self.dups += 1
        }
        *self.items.entry(v).or_default() += 1
    }

    pub fn remove(&mut self, v: T) {
        if let Some(&n) = self.items.get(&v) {
            if n == 1 {
                self.items.remove(&v);
                return;
            } else {
                self.dups -= 1;
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
