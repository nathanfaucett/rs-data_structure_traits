use super::collection::Collection;


pub trait Queue<T>: Collection {
    fn enqueue(&mut self, element: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}
