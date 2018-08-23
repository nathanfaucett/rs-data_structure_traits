use core::borrow::Borrow;

use super::{CollectionMut, GetMut, Insert, Map, Remove};

pub trait MapMut<'a, K, Q, V>:
    Map<'a, K, Q, V> + CollectionMut + GetMut<&'a Q, Output = V> + Insert<K, V> + Remove<K>
where
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{
}

impl<'a, K, Q, V, T> MapMut<'a, K, Q, V> for T
where
    T: 'a + Map<'a, K, Q, V> + CollectionMut + GetMut<&'a Q, Output = V> + Insert<K, V> + Remove<K>,
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{}
