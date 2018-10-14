extern crate gs;

use gs::JaroWinkler;
use std::time::Instant;

fn main() {
    let jw = JaroWinkler::new();
    for _ in 1..10 {
        let start = Instant::now();
        let mut n = 0.0;

        for _ in 0..10_000_000 {
            n += jw.apply("ALAKRAYINVESTBANK", "OAOKRAYINVESTBANK");
        }

        println!("Score: {} in {:?}", n, Instant::now().duration_since(start));
    }
}
