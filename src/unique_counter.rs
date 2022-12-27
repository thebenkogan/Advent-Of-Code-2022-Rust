// My implementation of a Counter data structure that efficiently keeps track
// of the uniqueness of the elements. All operations run in at most amortized
// constant time.

// Used in day 6 to create an efficient sliding window that maintains uniqueness.

use std::collections::HashMap;
use std::hash::Hash;

pub struct UniqCounter<T> {
    items: HashMap<T, i32>, // entry -> count of entry
    load: i32,              // total number of counted entries
}

impl<T: Eq + Hash> UniqCounter<T> {
    pub fn new(capacity: usize) -> Self {
        UniqCounter {
            items: HashMap::with_capacity(capacity),
            load: 0,
        }
    }

    pub fn is_unique(&self) -> bool {
        self.load == self.items.len() as i32
    }

    pub fn add(&mut self, v: T) {
        self.load += 1;
        *self.items.entry(v).or_default() += 1
    }

    pub fn remove(&mut self, v: T) {
        if let Some(&n) = self.items.get(&v) {
            self.load -= 1;
            if n == 1 {
                self.items.remove(&v);
                return;
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
