#[async_trait]
pub trait Scheduler<T>: Send + Sync + 'static {
    async fn add(&mut self, next: T);
    async fn next(&mut self);
    async fn execute(&mut self);
    async fn current(&mut self) -> T;
}
