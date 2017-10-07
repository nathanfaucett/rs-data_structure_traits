use core::borrow::Borrow;

use super::collection::Collection;
use super::iterable::Iterable;


pub trait Set<'a, V, Q>:
    Collection +
    Iterable<'a, &'a V>

    where V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{
    fn contains(&self, &Q) -> bool;
}
