use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn cci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("CCI");
    
    // Different data sizes with standard config
    group.bench_function(BenchmarkId::new("14-sma-stddev", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-sma-stddev", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-sma-stddev", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci();
            black_box(result);
        })
    });
    
    // Different configurations with full dataset
    group.bench_function(BenchmarkId::new("14-ema-stddev", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci_exp();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-sma-mad", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci_mad();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("20-sma-stddev", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_cci_20();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, cci_benchmark);
criterion_main!(benches);
