use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn macd_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("MACD");
    
    // Different data sizes with 7/14 SMA config
    group.bench_function(BenchmarkId::new("7-14-sma", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("7-14-sma", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("7-14-sma", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd();
            black_box(result);
        })
    });
    
    // Different configurations with full dataset
    group.bench_function(BenchmarkId::new("7-14-ema", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd_exp();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("12-26-ema-standard", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_macd_standard();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, macd_benchmark);
criterion_main!(benches);
