

pub trait QueueImmut<T> {
    fn enqueue(&self, T) -> Self;
    fn dequeue(&self) -> Self;
    fn peek(&self) -> Option<&T>;
}
