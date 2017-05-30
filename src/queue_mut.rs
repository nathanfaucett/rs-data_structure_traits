use super::collection_mut::CollectionMut;


pub trait QueueMut<T>: CollectionMut {
    fn enqueue(&mut self, element: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
}
