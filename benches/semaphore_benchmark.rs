use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use criterion::async_executor::FuturesExecutor;
use rust_semaphore::MySemaphore;
use std::time::Duration;

async fn acquire_semaphore(semaphore: &MySemaphore) {
    let _permit = semaphore.acquire().await.unwrap();
}

fn semaphore_benchmark(c: &mut Criterion) {
    let semaphore = MySemaphore::new(10);

    let mut group = c.benchmark_group("semaphore_benchmark");
    group
        .measurement_time(Duration::new(5, 0))
        .sample_size(10)
        .warm_up_time(Duration::new(3, 0));

    group.bench_function(BenchmarkId::new("semaphore acquire", 10), |b| {
        b.to_async(FuturesExecutor).iter(|| acquire_semaphore(&semaphore));
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().save_baseline("new".to_string());
    targets = semaphore_benchmark
}
criterion_main!(benches);
