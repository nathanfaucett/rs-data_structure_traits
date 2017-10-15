use core::borrow::Borrow;
use core::hash::{Hash, BuildHasher};
use std::collections::hash_map::HashMap;

use super::super::*;


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

impl<K, V> Create<(K, V)> for HashMap<K, V>
    where K: Eq + Hash,
{

    #[inline(always)]
    fn create() -> Self { HashMap::<K, V>::new() }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self { HashMap::<K, V>::new() }

    #[inline(always)]
    fn add_element(mut self, (key, value): (K, V)) -> Self {
        HashMap::<K, V>::insert(&mut self, key, value);
        self
    }
}

impl<'a, K, V, S> Insert<K, V> for HashMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Output = Option<V>;

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Self::Output {
        HashMap::<K, V, S>::insert(self, k, v)
    }
}

impl<'a, K, Q: ?Sized, V, S> Remove<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, q: &Q) -> Self::Output {
        HashMap::<K, V, S>::remove(self, q)
    }
}

impl<'a, K, Q: ?Sized, V, S> Get<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, q: &Q) -> Option<&Self::Output> {
        HashMap::<K, V, S>::get(self, q)
    }
}
impl<'a, K, Q: ?Sized, V, S> GetMut<&'a Q> for HashMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher,
{
    #[inline(always)]
    fn get_mut(&mut self, q: &Q) -> Option<&mut Self::Output> {
        HashMap::<K, V, S>::get_mut(self, q)
    }
}
