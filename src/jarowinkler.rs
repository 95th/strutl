use std::cmp;

/// A simple mutable implementation of Jaro-Winkler to
/// keep memory allocations minimum.
pub struct JaroWinkler {
    empty: Vec<bool>,
    min_flags: Vec<bool>,
    max_flags: Vec<bool>,
}

impl JaroWinkler {
    pub fn new() -> JaroWinkler {
        JaroWinkler::with_size(128)
    }

    pub fn with_size(size: usize) -> Self {
        assert_ne!(size, 0);
        JaroWinkler {
            empty: vec![false; size],
            min_flags: vec![false; size],
            max_flags: vec![false; size],
        }
    }

    /// Match two input strings and produces a score between 0 and 1.
    pub fn apply(&mut self, s1: &str, s2: &str) -> f64 {
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

    fn ensure_capacity(&mut self, capacity: usize) {
        let current_capacity = self.empty.len();
        if capacity <= current_capacity {
            return;
        }

        let mut new_capacity = current_capacity << 1;
        if new_capacity < capacity {
            new_capacity = capacity;
        }
        self.empty = vec![false; new_capacity];
        self.min_flags = vec![false; new_capacity];
        self.max_flags = vec![false; new_capacity];
    }

    fn calculate(&mut self, min: &[u8], max: &[u8]) -> f64 {
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

    fn matches(&mut self, min: &[u8], max: &[u8]) -> f64 {
        let range = cmp::max(max.len() / 2 - 1, 0);
        let mut matches = 0.0;

        for i in 0..min.len() {
            let c = min[i];

            let start = if i > range { i - range } else { 0 };
            let end = cmp::min(i + range + 1, max.len());

            for j in start..end {
                if !self.max_flags[j] && c == max[j] {
                    self.min_flags[i] = true;
                    self.max_flags[j] = true;
                    matches += 1.0;
                    break;
                }
            }
        }
        matches
    }

    fn transpositions(&mut self, min: &[u8], max: &[u8]) -> f64 {
        let mut t = 0;
        let mut j = 0;

        for i in 0..min.len() {
            if !self.min_flags[i] {
                continue;
            }
            while j < max.len() && !self.max_flags[j] {
                j += 1;
            }
            if min[i] != max[j] {
                t += 1;
            }
            j += 1;
        }

        self.min_flags[..min.len()].copy_from_slice(&self.empty[..min.len()]);
        self.max_flags[..max.len()].copy_from_slice(&self.empty[..max.len()]);

        (t / 2) as f64
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

#[cfg(test)]
mod tests {
    use super::JaroWinkler;

    #[test]
    fn partial_match() {
        let mut jw = JaroWinkler::new();
        let score = jw.apply("Foo bar", "Food candybar");
        assert_eq!(score, 0.7897435897435898);
    }

    #[test]
    fn full_match() {
        let mut jw = JaroWinkler::new();
        let score = jw.apply("Foo bar", "Foo bar");
        assert_eq!(score, 1.0);
    }

    #[test]
    fn no_match() {
        let mut jw = JaroWinkler::new();
        let score = jw.apply("Foobar", "pqxyz");
        assert_eq!(score, 0.0);
    }
}
