use super::collection::Collection;


pub trait Deque<T>: Collection {
    fn push_front(&self, element: T) -> Self;
    fn push_back(&self, element: T) -> Self;

    fn pop_front(&self) -> Self;
    fn pop_back(&self) -> Self;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;
}
