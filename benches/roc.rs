use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn roc_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RateOfChange");
    
    // Different data sizes
    group.bench_function(BenchmarkId::new("roc", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_roc_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("roc", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_roc_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("roc", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_roc();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, roc_benchmark);
criterion_main!(benches);
