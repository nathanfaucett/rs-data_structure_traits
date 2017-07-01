use core::borrow::Borrow;

use super::collection::Collection;
use super::get::Get;
use super::insert::Insert;
use super::remove::Remove;
use super::iterable::Iterable;


pub trait Map<'a, K, Q: ?Sized, V>:
    Collection +

    Insert<K, V, Output = Option<V>> +
    Remove<&'a Q> +

    Get<&'a Q, Output = V> +

    Iterable<'a, (&'a K, &'a V)>

    where K: 'a + Borrow<Q>,
          Q: 'a,
          V: 'a
{
    fn contains_key(&self, k: &Q) -> bool;
}
