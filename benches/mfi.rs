use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustti_benchmarks;
use std::hint::black_box;

fn mfi_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("MoneyFlowIndex");
    
    // Different data sizes with 14-period
    group.bench_function(BenchmarkId::new("14-period", "small-100"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mfi_small();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period", "medium-1000"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mfi_medium();
            black_box(result);
        })
    });
    
    group.bench_function(BenchmarkId::new("14-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mfi();
            black_box(result);
        })
    });
    
    // Different period length
    group.bench_function(BenchmarkId::new("7-period", "large-2552"), |b| {
        b.iter(|| {
            let result = rustti_benchmarks::compute_mfi_7();
            black_box(result);
        })
    });
    
    group.finish();
}

criterion_group!(benches, mfi_benchmark);
criterion_main!(benches);
