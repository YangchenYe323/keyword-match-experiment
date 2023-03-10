#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use keyword_match_experiment::{
    match_keyword_perfect_hash, match_keyword_baseline, match_keyword_rust_custom_hash, match_keyword_rust_hash,
    simd_str_match, str_match,
    utils::{generate_keywords, read_keys_from_file},
};

fn bench_string_match(criterion: &mut Criterion) {
    // Randomly generate two byte patterns for comparison.
    let candidates = generate_keywords(1);
    let keywords = generate_keywords(1);
    let (c, k) = (candidates[0].as_slice(), keywords[0].as_slice());
    let c = unsafe { std::str::from_utf8_unchecked(c) };
    let k = unsafe { std::str::from_utf8_unchecked(k) };

    // Uses `==` operator
    let mut group = criterion.benchmark_group("string-match-baseline");
    group.bench_with_input(BenchmarkId::new("baseline", 1), &(c, k), |b, (c, k)| {
        b.iter(|| str_match(*c, *k));
    });
    group.finish();

    // Uses Simd
    let mut group2 = criterion.benchmark_group("string-match-simd");
    group2.bench_with_input(BenchmarkId::new("simd", 1), &(c, k), |b, (c, k)| {
        b.iter(|| simd_str_match(*c, *k));
    });
    group2.finish();
}

fn bench_keyword_match(c: &mut Criterion) {
    let candidates = read_keys_from_file("mixed.txt");
    let len = candidates.len();
    let candidates: Vec<&str> = candidates
        .iter()
        .map(|v| unsafe { std::str::from_utf8_unchecked(v) })
        .collect();
    let param = format!("#words matched: {}", len);

    let mut group = c.benchmark_group("Match JS Keywords");
    group.bench_function(
        BenchmarkId::new("Rust HashMap(default hasher)", &param),
        |b| {
            b.iter(|| {
                for c in &candidates {
                    black_box(match_keyword_rust_hash(c));
                }
            });
        },
    );
    group.bench_function(
        BenchmarkId::new("Rust HashMap(custom hasher)", &param),
        |b| {
            b.iter(|| {
                for c in &candidates {
                    black_box(match_keyword_rust_custom_hash(c));
                }
            });
        },
    );
    group.bench_function(BenchmarkId::new("Match Statement", &param), |b| {
        b.iter(|| {
            for c in &candidates {
                black_box(match_keyword_baseline(c));
            }
        });
    });
    group.bench_function(BenchmarkId::new("Perfect Hash Table", &param), |b| {
        b.iter(|| {
            for c in &candidates {
                black_box(match_keyword_perfect_hash(c));
            }
        });
    });
}

criterion_group!(benches, bench_keyword_match);
criterion_main!(benches);
