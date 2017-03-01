use core::iter::Iterator;


pub trait Iterable<'a> {
    type Iter: Iterator;

    fn iter(&'a self) -> Self::Iter;
}
