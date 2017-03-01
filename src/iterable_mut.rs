use core::iter::Iterator;


pub trait IterableMut<'a> {
    type Output: Iterator;

    fn iter_mut(&'a mut self) -> Self::Output;
}
