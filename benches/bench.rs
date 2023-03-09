use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simd_string_match::{simd_str_match, str_match, ELEMENTS};

fn generate_keywords(n: usize) -> Vec<[u8; ELEMENTS]> {
    use rand::RngCore;
    let mut vec = vec![[0; 16]; n];
    for i in 0..n {
        let byte = &mut vec[i];
        rand::thread_rng().fill_bytes(byte);
    }

    vec
}

lazy_static::lazy_static! {
  pub static ref KEYWORDS: Vec<[u8; 16]> = generate_keywords(1);
  pub static ref CANDIDATES: Vec<[u8; 16]> = generate_keywords(1);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline");
    let mut cnt = 0;
    for c in CANDIDATES.iter() {
        for k in KEYWORDS.iter() {
            let c = unsafe { std::str::from_utf8_unchecked(c) };
            let k = unsafe { std::str::from_utf8_unchecked(k) };
            group.bench_with_input(BenchmarkId::new("baseline", cnt), &(c, k), |b, (c, k)| {
                b.iter(|| str_match(*c, *k));
            });
            cnt += 1;
        }
    }
    group.finish();

    let mut group2 = c.benchmark_group("simd");
    cnt = 0;
    for c in CANDIDATES.iter() {
        for k in KEYWORDS.iter() {
            let c = unsafe { std::str::from_utf8_unchecked(c) };
            let k = unsafe { std::str::from_utf8_unchecked(k) };
            group2.bench_with_input(BenchmarkId::new("simd", cnt), &(c, k), |b, (c, k)| {
                b.iter(|| simd_str_match(*c, *k));
            });
            cnt += 1;
        }
    }
    group2.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
