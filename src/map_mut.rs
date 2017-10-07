use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::iterable_mut::IterableMut;
use super::get_mut::GetMut;
use super::map::Map;


pub trait MapMut<'a, K, Q, V>:
    Map<'a, K, Q, V> +

    CollectionMut +

    GetMut<&'a Q, Output = V> +

    IterableMut<'a, (&'a K, &'a mut V)>

    where K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{}
