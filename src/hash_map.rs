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
        HashMap::<K, V, S>::len(self)
    }
}

impl<K, V, S> CollectionMut for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn clear(&mut self) {
        HashMap::<K, V, S>::drain(self);
    }
}

impl<K, V> Create for HashMap<K, V>
    where K: Eq + Hash
{
    #[inline(always)]
    fn create() -> Self {
        HashMap::<K, V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self {
        HashMap::<K, V>::with_capacity(capacity)
    }
}

impl<K, V, S> AddElementMut<(K, V)> for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn add_element(&mut self, (key, value): (K, V)) {
        HashMap::<K, V, S>::insert(self, key, value);
    }
}

impl<'a, K, V, S> InsertMut<K, V> for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Output = Option<V>;

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Self::Output {
        HashMap::<K, V, S>::insert(self, k, v)
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
        HashMap::<K, V, S>::remove(self, k)
    }
}

impl<'a, K, Q: ?Sized, V, S> Get<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> {
        HashMap::get(self, k)
    }
}
impl<'a, K, Q: ?Sized, V, S> GetMut<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher,
{
    #[inline(always)]
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> {
        HashMap::get_mut(self, k)
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
        HashMap::<K, V, S>::iter(self)
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
        HashMap::<K, V, S>::iter_mut(self)
    }
}

impl<'a, K, Q: ?Sized, V, S> MapMut<'a, K, Q, V> for HashMap<K, V, S>
    where K: 'a + Eq + Hash + Borrow<Q>,
          Q: 'a + Eq + Hash,
          V: 'a,
          S: 'a + BuildHasher,
{
    #[inline(always)]
    fn contains_key(&self, k: &Q) -> bool {
        HashMap::contains_key(self, k)
    }
}
