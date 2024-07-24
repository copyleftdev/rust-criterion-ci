use rust_semaphore::MySemaphore;
use tokio::runtime::Runtime;

#[test]
fn test_semaphore() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let semaphore = MySemaphore::new(1);
        
        // Acquire the semaphore
        {
            let permit1 = semaphore.acquire().await;
            assert!(permit1.is_ok());
        } // The permit1 is dropped here, releasing the semaphore
        
        // Attempt to acquire the semaphore again
        let permit2 = semaphore.acquire().await;
        assert!(permit2.is_ok());
    });
}
