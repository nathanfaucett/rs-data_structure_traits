

pub trait Deque<T> {
    fn push_front(&self, T) -> Self;
    fn push_back(&self, T) -> Self;

    fn pop_front(&self) -> Self;
    fn pop_back(&self) -> Self;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;
}
