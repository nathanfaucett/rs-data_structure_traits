use core::hash::{Hash, BuildHasher};
use std::collections::hash_set::HashSet;

use super::*;


impl<V, S> Collection for HashSet<V, S>
    where V: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn len(&self) -> usize {
        HashSet::<V, S>::len(self)
    }
}

impl<V, S> CollectionMut for HashSet<V, S>
    where V: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn clear(&mut self) {
        HashSet::<V, S>::drain(self);
    }
}

impl<V> Create for HashSet<V>
    where V: Eq + Hash
{
    #[inline(always)]
    fn create() -> Self {
        HashSet::<V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self {
        HashSet::<V>::with_capacity(capacity)
    }
}

impl<V, S> AddElementMut<V> for HashSet<V, S>
    where V: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn add_element(&mut self, element: V) {
        HashSet::<V, S>::insert(self, element);
    }
}
