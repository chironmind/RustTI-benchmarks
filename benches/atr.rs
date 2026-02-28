use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn atr_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AverageTrueRange");
    
    // Different data sizes with 5-period SMA
    group.bench_function(BenchmarkId::new("5-sma", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-sma", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-sma", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr();
            black_box(result);
        })
    });
    
    // Different configurations with full dataset
    group.bench_function(BenchmarkId::new("14-sma", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr_14();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-ema", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_atr_exp();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, atr_benchmark);
criterion_main!(benches);
