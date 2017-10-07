use core::iter::Iterator;


pub trait IterableMut<'a, T: 'a> {
    type IterMut: Iterator<Item = T>;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}
