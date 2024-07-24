use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct MySemaphore {
    semaphore: Arc<Semaphore>,
}

impl MySemaphore {
    pub fn new(size: usize) -> Self {
        MySemaphore {
            semaphore: Arc::new(Semaphore::new(size)),
        }
    }

    pub async fn acquire(&self) -> Result<tokio::sync::OwnedSemaphorePermit, tokio::sync::AcquireError> {
        self.semaphore.clone().acquire_owned().await
    }
}
