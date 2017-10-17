

pub trait QueueImmut<T>: Sized {
    fn enqueue(&self, T) -> Self;
    fn dequeue(&self) -> Self;
    fn peek(&self) -> Option<&T>;

    #[inline(always)]
    fn dequeue_and_peek(&self) -> (Self, Option<&T>) {
        (self.dequeue(), self.peek())
    }
}
