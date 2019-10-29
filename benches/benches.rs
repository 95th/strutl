use criterion::{black_box, criterion_group, criterion_main, Criterion};
use text::JaroWinkler;

const INPUTS: &'static [(&'static str, &'static str)] = &[
    ("Hello world", "Hi world"),
    ("a", "b"),
    ("a", "abcdefgjlkasdflksldf"),
    ("abcd", "abcdefgh"),
    ("abcd", "abcdefghijkl"),
    ("abcd", "abcd"),
    ("abcd", "abdc"),
    ("abcd", "acbd"),
    ("abcd", "afgh"),
    ("ijsdfkjksjdfkjskdfjksdjf", "kaskdjfksjdkfjskdfjksdf"),
    ("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxzy"),
];

fn large_input() -> Vec<(String, String)> {
    (0..10)
        .flat_map(|i| {
            vec![
                (format!("Hello world {}", i), format!("Hi world {}", i + 1)),
                (format!("a {}", i), format!("b {}", i + 1)),
                (
                    format!("a {}", i),
                    format!("abcdefgjlkasdflksldf {}", i + 1),
                ),
                (format!("abcd {}", i), format!("abcdefgh {}", i + 1)),
                (format!("abcd {}", i), format!("abcdefghijkl {}", i + 1)),
                (format!("abcd {}", i), format!("abcd {}", i + 1)),
                (format!("abcd {}", i), format!("abdc {}", i + 1)),
                (format!("abcd {}", i), format!("acbd {}", i + 1)),
                (format!("abcd {}", i), format!("afgh {}", i + 1)),
                (
                    format!("ijsdfkjksjdfkjskdfjksdjf {}", i),
                    format!("kaskdjfksjdkfjskdfjksdf {}", i + 1),
                ),
                (
                    format!("abcdefghijklmnopqrstuvwxyz {}", i),
                    format!("abcdefghijklmnopqrstuvwxzy {}", i + 1),
                ),
            ]
        })
        .collect()
}

pub fn jaro(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "jaro",
        |b, (s, t)| {
            let mut j = JaroWinkler::new();
            b.iter(|| black_box(j.apply(s, t)));
        },
        INPUTS,
    );
}

pub fn large(c: &mut Criterion) {
    c.bench_function("large", |b| {
        let mut j = JaroWinkler::new();
        let input = black_box(large_input());
        b.iter(|| {
            for (s, t) in &input {
                black_box(j.apply(s, t));
            }
        });
    });
}

criterion_group!(benches, jaro, large);
criterion_main!(benches);
