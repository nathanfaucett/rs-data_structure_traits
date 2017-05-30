use super::collection::Collection;


pub trait Stack<T>: Collection {
    fn push(&self, element: T) -> Self;
    fn pop(&self) -> Self;

    fn top(&self) -> Option<&T>;
}
