use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn obv_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("OnBalanceVolume");
    
    // Different data sizes
    group.bench_function(BenchmarkId::new("obv", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_obv_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("obv", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_obv_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("obv", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_obv();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, obv_benchmark);
criterion_main!(benches);
