macro_rules! unsafe_assert {
    ($condition: expr) => {
        if !$condition {
            if cfg!(debug_assertions) {
                panic!("Assertion failed: {}", stringify!($condition));
            } else {
                unsafe { std::hint::unreachable_unchecked() };
            }
        }
    };
}

/// A simple mutable implementation of Jaro-Winkler to
/// keep memory allocations minimum.
#[derive(Default)]
pub struct JaroWinkler {
    indices: Vec<isize>,
}

impl JaroWinkler {
    pub fn new() -> Self {
        JaroWinkler::with_size(128)
    }

    pub fn with_size(size: usize) -> Self {
        JaroWinkler {
            indices: vec![-1; size],
        }
    }

    /// Match two input strings and produces a score between 0 and 1.
    pub fn apply(&mut self, s1: impl AsRef<[u8]>, s2: impl AsRef<[u8]>) -> f64 {
        let mut b1 = s1.as_ref();
        let mut b2 = s2.as_ref();
        if b1.is_empty() && b2.is_empty() {
            return 1.0;
        }

        if b1.is_empty() || b2.is_empty() {
            return 0.0;
        }

        self.ensure_capacity(b1.len() + b2.len());

        if b1.len() > b2.len() {
            std::mem::swap(&mut b1, &mut b2);
        }
        self.calculate(b1, b2)
    }

    fn ensure_capacity(&mut self, capacity: usize) {
        let current_capacity = self.indices.len();
        if capacity <= current_capacity {
            return;
        }

        let mut new_capacity = current_capacity * 2;
        if new_capacity < capacity {
            new_capacity = capacity;
        }
        self.indices = vec![-1; new_capacity];
    }

    fn calculate(&mut self, min: &[u8], max: &[u8]) -> f64 {
        let mut inner = Inner::new(&mut self.indices, min, max);
        let m = inner.matches();

        if m == 0 {
            return 0.0;
        }

        let t = inner.transpositions(m) as f64;
        let p = inner.prefix() as f64;
        let min_len = min.len() as f64;
        let max_len = max.len() as f64;
        let m = m as f64;

        let j = (m / min_len + m / max_len + (m - t) / m) / 3.0;
        j + 0.1 * p * (1.0 - j)
    }
}

struct Inner<'a> {
    min_indices: &'a mut [isize],
    max_flags: &'a mut [isize],
    min: &'a [u8],
    max: &'a [u8],
}

impl<'a> Inner<'a> {
    fn new(indices: &'a mut [isize], min: &'a [u8], max: &'a [u8]) -> Self {
        use std::slice::from_raw_parts_mut;
        let ptr = indices.as_mut_ptr();

        // Safety: we ensured that both `min` and `max` are non-zero length in `JaroWinkler::apply` method
        // and `indices` is at least as large as `min.len() + max.len()` in `JaroWinkler::ensure_capacity`
        unsafe {
            let min_indices = from_raw_parts_mut(ptr, min.len());
            let max_flags = from_raw_parts_mut(ptr.add(min.len()), max.len());

            Self {
                min_indices,
                max_flags,
                min,
                max,
            }
        }
    }

    fn matches(&mut self) -> usize {
        let range = (self.max.len() / 2).saturating_sub(1);
        let mut idx = 0;
        for i in 0..self.min.len() {
            let c1 = self.min[i];
            let start = i.saturating_sub(range);
            let end = self.max.len().min(i + range + 1);

            for j in start..end {
                let c2 = self.max[j];
                if c1 == c2 && self.max_flags[j] != 0 {
                    unsafe_assert! { idx < self.min_indices.len() };
                    self.min_indices[idx] = i as isize;
                    self.max_flags[j] = 0;
                    idx += 1;
                    break;
                }
            }
        }
        idx
    }

    fn transpositions(&mut self, matches: usize) -> usize {
        let mut t = 0;
        let mut max_index = 0;

        for i in 0..matches {
            unsafe_assert! { i < self.min_indices.len() };
            let min_index = self.min_indices[i] as usize;

            loop {
                unsafe_assert! { max_index < self.max_flags.len() };
                if self.max_flags[max_index] == 0 {
                    break;
                }
                max_index += 1;
            }

            unsafe_assert! { min_index < self.min.len() };
            if self.min[min_index] != self.max[max_index] {
                t += 1;
            }

            self.max_flags[max_index] = -1;
            self.min_indices[i] = -1;
            max_index += 1;
        }

        t / 2
    }

    fn prefix(&self) -> usize {
        self.min
            .iter()
            .zip(self.max.iter())
            .take(4)
            .take_while(|(a, b)| a == b)
            .count()
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
