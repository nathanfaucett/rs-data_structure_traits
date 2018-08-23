use core::borrow::Borrow;

use super::{CollectionMut, GetMut, InsertMut, Map, RemoveMut};

pub trait MapMut<'a, K, Q, V>:
    Map<'a, K, Q, V>
    + CollectionMut
    + GetMut<&'a Q, Output = V>
    + InsertMut<K, V, Output = Option<V>>
    + RemoveMut<K, Output = Option<V>>
where
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{
}

impl<'a, K, Q, V, T> MapMut<'a, K, Q, V> for T
where
    T: 'a
        + Map<'a, K, Q, V>
        + CollectionMut
        + GetMut<&'a Q, Output = V>
        + InsertMut<K, V, Output = Option<V>>
        + RemoveMut<K, Output = Option<V>>,
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{}
