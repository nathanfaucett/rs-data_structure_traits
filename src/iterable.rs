use core::iter::Iterator;


pub trait Iterable<'a> {
    type Output: Iterator;

    fn iter(&'a self) -> Self::Output;
}
