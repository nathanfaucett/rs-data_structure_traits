use core::borrow::Borrow;

use super::collection::Collection;
use super::get::Get;
use super::iterable::Iterable;


pub trait Map<'a, K, Q, V>:
    Collection +

    Get<&'a Q, Output = V> +

    Iterable<'a, (&'a K, &'a V)>

    where K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{
    fn contains_key(&self, &Q) -> bool;
}
