use rust_semaphore::MySemaphore;
use tokio::runtime::Runtime;

#[test]
fn test_semaphore() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let semaphore = MySemaphore::new(1);
        let permit1 = semaphore.acquire().await;
        let permit2 = semaphore.acquire().await;
        assert!(permit1.is_some());
        assert!(permit2.is_some());
    });
}
