use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn rsi_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RSI");
    
    // Standard configuration with different data sizes
    group.bench_function(BenchmarkId::new("14-period-smoothed", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period-smoothed", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period-smoothed", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi();
            black_box(result);
        })
    });
    
    // Different moving average types (with full dataset)
    group.bench_function(BenchmarkId::new("14-period-simple", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_simple();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period-exponential", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_exp();
            black_box(result);
        })
    });
    
    // Different period lengths
    group.bench_function(BenchmarkId::new("7-period-smoothed", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_7();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("21-period-smoothed", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_rsi_21();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, rsi_benchmark);
criterion_main!(benches);
