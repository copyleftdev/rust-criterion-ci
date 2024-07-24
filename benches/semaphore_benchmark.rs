use criterion::{criterion_group, criterion_main, Criterion};
use rust_semaphore::MySemaphore;
use tokio::runtime::Runtime;

fn semaphore_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let semaphore = MySemaphore::new(10);

    c.bench_function("semaphore acquire", |b| {
        b.to_async(&rt).iter(|| async {
            let _permit = semaphore.acquire().await;
        });
    });
}

criterion_group!(benches, semaphore_benchmark);
criterion_main!(benches);
