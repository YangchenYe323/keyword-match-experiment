use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use keyword_match_experiment::{
    match_keyword, match_keyword_baseline, simd_str_match, str_match, ELEMENTS,
    MAX_JS_KEYWORD_LENGTH, MIN_JS_KEYWORD_LENGTH,
};

fn generate_keywords(n: usize) -> Vec<Vec<u8>> {
    use rand::RngCore;
    let mut vec = vec![vec![0; ELEMENTS]; n];
    for i in 0..n {
        let byte = &mut vec[i];
        rand::thread_rng().fill_bytes(byte);
    }

    vec
}

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
    const CANDIDATE_LEN: usize = 8;
    // candidate contains a [u8; 16] buffer so that the SIMD algorithm can work correctly.
    let candidates = generate_keywords(CANDIDATE_LEN);
    // The baseline algorithm first checks whether the candidate's length falls in valid range,
    // so a 16-char str will be filtered out immediately. We don't want that to happen,
    // so we randomly pick a valid length so it has to look at the match arms.
    let lens: Vec<usize> = vec![
        rand::thread_rng()
            .gen_range(MIN_JS_KEYWORD_LENGTH..=MAX_JS_KEYWORD_LENGTH);
        CANDIDATE_LEN
    ];
    let candidates: Vec<&str> = candidates
        .iter()
        .zip(lens.iter())
        .map(|(c, l)| unsafe { std::str::from_utf8_unchecked(&c[..*l]) })
        .collect();

    let mut group = c.benchmark_group("keyword-match-baseline");
    group.bench_with_input(
        BenchmarkId::new("match_keyword_baseline", CANDIDATE_LEN),
        &candidates[..],
        |b, candidates| {
            b.iter(|| {
                for c in candidates {
                    match_keyword_baseline(c);
                }
            });
        },
    );
    group.finish();

    let mut group2 = c.benchmark_group("keyword-match-hash");
    group2.bench_with_input(
        BenchmarkId::new("match_keyword", CANDIDATE_LEN),
        &candidates[..],
        |b, candidates| {
            b.iter(|| {
                for c in candidates {
                    match_keyword(c);
                }
            });
        },
    );
    group2.finish();
}

criterion_group!(benches, bench_string_match, bench_keyword_match);
criterion_main!(benches);
