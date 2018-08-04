pub trait StackImmut<T>: Sized {
    fn push(&self, T) -> Self;
    fn pop(&self) -> Self;

    fn top(&self) -> Option<&T>;

    #[inline(always)]
    fn pop_and_top(&self) -> (Self, Option<&T>) {
        (self.pop(), self.top())
    }
}
