use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn mg_dyn_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("McGinleyDynamic");
    
    // Different data sizes with 5-period
    group.bench_function(BenchmarkId::new("5-period", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_dyn_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_dyn_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("5-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_dyn();
            black_box(result);
        })
    });
    
    // Different period length with full dataset
    group.bench_function(BenchmarkId::new("20-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mg_dyn_20();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, mg_dyn_benchmark);
criterion_main!(benches);
