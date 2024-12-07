use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use aoc_2024_01_2::compute_result;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main", |b| b.iter(|| compute_result()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
