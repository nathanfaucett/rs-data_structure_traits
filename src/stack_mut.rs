

pub trait StackMut<T> {
    fn push(&mut self, T);
    fn pop(&mut self) -> Option<T>;

    fn top(&self) -> Option<&T>;
    fn top_mut(&mut self) -> Option<&mut T>;
}
