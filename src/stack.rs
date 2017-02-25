use super::collection::Collection;


pub trait Stack<T>: Collection {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<&T>;
}
