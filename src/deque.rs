pub trait Deque<T> {
    fn push_front(&mut self, T);
    fn push_back(&mut self, T);

    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;

    fn front_mut(&mut self) -> Option<&mut T>;
    fn back_mut(&mut self) -> Option<&mut T>;
}
