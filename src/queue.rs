use super::collection::Collection;


pub trait Queue<T>: Collection {
    fn enqueue(&self, element: T) -> Self;
    fn dequeue(&self) -> Self;
    fn peek(&self) -> Option<&T>;
}
