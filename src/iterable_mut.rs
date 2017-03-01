use super::iterable::Iterable;


pub trait IterableMut<'a>: Iterable<'a> {
    fn iter_mut(&'a mut self) -> Self::Output;
}
