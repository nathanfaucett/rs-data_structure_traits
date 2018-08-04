use core::borrow::Borrow;

use super::{InsertImmut, Map, RemoveImmut};

pub trait MapImmut<'a, K, Q, V>: Map<'a, K, Q, V> + InsertImmut<K, V> + RemoveImmut<K>
+
where
    &'a Self: 'a + IntoIterator<Item = (&'a K, &'a V)>,
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{
}

impl<'a, K, Q, V, T> MapImmut<'a, K, Q, V> for T
where
    T: 'a + Map<'a, K, Q, V> + InsertImmut<K, V> + RemoveImmut<K>,
    &'a T: 'a + IntoIterator<Item = (&'a K, &'a V)>,
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + Sized,
{}
