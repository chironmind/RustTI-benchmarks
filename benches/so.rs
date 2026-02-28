use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn so_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("StochasticOscillator");
    
    // Different data sizes with 14-period
    group.bench_function(BenchmarkId::new("14-period", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_so_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_so_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_so();
            black_box(result);
        })
    });
    
    // Different period length
    group.bench_function(BenchmarkId::new("5-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_so_5();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, so_benchmark);
criterion_main!(benches);
