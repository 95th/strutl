use std::cell::RefCell;
use std::cmp;

pub struct JaroWinkler {
    empty: RefCell<Vec<bool>>,
    min_flags: RefCell<Vec<bool>>,
    max_flags: RefCell<Vec<bool>>,
}

impl JaroWinkler {
    pub fn new() -> JaroWinkler {
        JaroWinkler {
            empty: RefCell::new(vec![false; 128]),
            min_flags: RefCell::new(vec![false; 128]),
            max_flags: RefCell::new(vec![false; 128]),
        }
    }

    pub fn apply(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }

        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        if s1 == s2 {
            return 1.0;
        }

        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();

        let max_len = cmp::max(b1.len(), b2.len());
        self.ensure_capacity(max_len);

        if b1.len() > b2.len() {
            self.calculate(b2, b1)
        } else {
            self.calculate(b1, b2)
        }
    }

    fn ensure_capacity(&self, capacity: usize) {
        let current_capacity = self.empty.borrow().len();
        if capacity <= current_capacity {
            return;
        }

        let mut new_capacity = current_capacity << 1;
        if new_capacity < capacity {
            new_capacity = capacity;
        }
        self.empty.borrow_mut().resize(new_capacity, false);
        self.min_flags.borrow_mut().resize(new_capacity, false);
        self.max_flags.borrow_mut().resize(new_capacity, false);
    }

    fn calculate(&self, min: &[u8], max: &[u8]) -> f64 {
        let m = self.matches(min, max);

        if m == 0.0 {
            return 0.0;
        }

        let t = self.transpositions(min, max);
        let p = self.prefix(min, max);
        let min_len = min.len() as f64;
        let max_len = max.len() as f64;

        let j = (m / min_len + m / max_len + (m - t) / m) / 3.0;
        j + 0.1 * p * (1.0 - j)
    }

    fn matches(&self, min: &[u8], max: &[u8]) -> f64 {
        let range = cmp::max(max.len() / 2 - 1, 0);
        let mut matches = 0.0;

        let mut min_flags = self.min_flags.borrow_mut();
        let mut max_flags = self.max_flags.borrow_mut();

        for i in 0..min.len() {
            let c = min[i];

            let start = if i > range { i - range } else { 0 };
            let end = cmp::min(i + range + 1, max.len());

            for j in start..end {
                if !max_flags[j] && c == max[j] {
                    min_flags[i] = true;
                    max_flags[j] = true;
                    matches += 1.0;
                    break;
                }
            }
        }
        matches
    }

    fn transpositions(&self, min: &[u8], max: &[u8]) -> f64 {
        let mut transpositions = 0.0;
        let mut j = 0;

        let mut min_flags = self.min_flags.borrow_mut();
        let mut max_flags = self.max_flags.borrow_mut();

        for i in 0..min.len() {
            if !min_flags[i] {
                continue;
            }
            while j < max.len() && !max_flags[j] {
                j += 1;
            }
            if min[i] != max[j] {
                transpositions += 0.5;
            }
            j += 1;
        }

        let empty = &self.empty.borrow();
        min_flags[0..max.len()].copy_from_slice(&empty[0..max.len()]);
        max_flags[0..max.len()].copy_from_slice(&empty[0..max.len()]);

        transpositions
    }

    fn prefix(&self, min: &[u8], max: &[u8]) -> f64 {
        let mut prefix = 0.0;
        for i in 0..cmp::min(min.len(), 4) {
            if min[i] == max[i] {
                prefix += 1.0;
            } else {
                break;
            }
        }
        prefix
    }
}
