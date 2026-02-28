use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn ema_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ExponentialMovingAverage");
    
    // Different data sizes with 5-period
    group.bench_function(BenchmarkId::new("5-period", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema();
            black_box(result);
        })
    });
    
    // Standard MACD periods with full dataset
    group.bench_function(BenchmarkId::new("12-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema_12();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("26-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_ema_26();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, ema_benchmark);
criterion_main!(benches);
