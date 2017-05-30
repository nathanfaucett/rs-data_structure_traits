use super::collection_mut::CollectionMut;


pub trait StackMut<T>: CollectionMut {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;

    fn top(&self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
}
