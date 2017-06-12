use core::borrow::Borrow;
use core::hash::{Hash, BuildHasher};
use std::collections::hash_map::{HashMap, Iter, IterMut};

use super::*;


impl<K, V, S> Collection for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> CollectionMut for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn clear(&mut self) {
        self.drain();
    }
}

impl<'a, K, V, S> InsertMut<K, V> for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Output = Option<V>;

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Self::Output {
        self.insert(k, v)
    }
}

impl<'a, K, Q: ?Sized, V, S> RemoveMut<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, k: &Q) -> Self::Output {
        self.remove(k)
    }
}

impl<'a, K, V, S> Iterable<'a, (&'a K, &'a V)> for HashMap<K, V, S>
    where K: 'a + Eq + Hash,
          V: 'a,
          S: 'a + BuildHasher,
{
    type Iter = Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, K, V, S> IterableMut<'a, (&'a K, &'a mut V)> for HashMap<K, V, S>
    where K: 'a + Eq + Hash,
          V: 'a,
          S: 'a + BuildHasher,
{
    type IterMut = IterMut<'a, K, V>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.iter_mut()
    }
}
