

pub trait Queue<T> {
    fn enqueue(&mut self, T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}
