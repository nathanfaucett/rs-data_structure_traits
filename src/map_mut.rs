use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::insert_mut::InsertMut;
use super::remove_mut::RemoveMut;
use super::iterable::Iterable;
use super::iterable_mut::IterableMut;
use super::get::Get;
use super::get_mut::GetMut;


pub trait MapMut<'a, K, Q: ?Sized, V>:
    CollectionMut +

    InsertMut<K, V, Output = Option<V>> +
    RemoveMut<&'a Q, Output = Option<V>> +

    Get<&'a Q, Output = V> +
    GetMut<&'a Q, Output = V> +

    Iterable<'a, (&'a K, &'a V)> +
    IterableMut<'a, (&'a K, &'a mut V)>

    where K: 'a + Borrow<Q>,
          Q: 'a,
          V: 'a
{
    fn contains_key(&self, k: &Q) -> bool;
}
