use core::iter::Iterator;


pub trait Iterable<'a, T> {
    type Iter: Iterator<Item = T>;

    fn iter(&'a self) -> Self::Iter;
}
