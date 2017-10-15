

pub trait StackImmut<T> {
    fn push(&self, T) -> Self;
    fn pop(&self) -> Self;

    fn top(&self) -> Option<&T>;
}
