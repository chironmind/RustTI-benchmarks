use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn log_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Log");
    
    // Different data sizes
    group.bench_function(BenchmarkId::new("log", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_log_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("log", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_log_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("log", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_log();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, log_benchmark);
criterion_main!(benches);
