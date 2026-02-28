use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn slowest_so_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("SlowestStochastic");
    
    // Different data sizes with 14-period SMA
    group.bench_function(BenchmarkId::new("14-sma", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_slowest_so_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-sma", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_slowest_so_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-sma", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_slowest_so();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, slowest_so_benchmark);
criterion_main!(benches);
