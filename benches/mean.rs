use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn mean_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mean");
    
    // Different data sizes with 5-period
    group.bench_function(BenchmarkId::new("5-period", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mean_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mean_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mean();
            black_box(result);
        })
    });
    
    // Different period length
    group.bench_function(BenchmarkId::new("20-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mean_20();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, mean_benchmark);
criterion_main!(benches);
