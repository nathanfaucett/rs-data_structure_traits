use core::iter::Iterator;


pub trait IterableMut<'a> {
    type IterMut: Iterator;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}
