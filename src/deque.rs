pub trait Deque<T>: Sized {
    fn push_front(&self, T) -> Self;
    fn push_back(&self, T) -> Self;

    fn pop_front(&self) -> Self;
    fn pop_back(&self) -> Self;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;

    #[inline(always)]
    fn pop_and_front(&self) -> (Self, Option<&T>) {
        (self.pop_front(), self.front())
    }
    #[inline(always)]
    fn pop_and_back(&self) -> (Self, Option<&T>) {
        (self.pop_back(), self.back())
    }
}
