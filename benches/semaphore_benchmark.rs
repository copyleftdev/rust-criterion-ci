use criterion::{criterion_group, criterion_main, Criterion};
use criterion::async_executor::FuturesExecutor;
use rust_semaphore::MySemaphore;

async fn acquire_semaphore(semaphore: &MySemaphore) {
    let _permit = semaphore.acquire().await.unwrap();
}

fn semaphore_benchmark(c: &mut Criterion) {
    let semaphore = MySemaphore::new(10);

    c.bench_function("semaphore acquire", |b| {
        b.to_async(FuturesExecutor).iter(|| acquire_semaphore(&semaphore));
    });
}

criterion_group!(benches, semaphore_benchmark);
criterion_main!(benches);
