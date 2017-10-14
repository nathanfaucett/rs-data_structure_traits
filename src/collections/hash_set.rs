use core::hash::{Hash, BuildHasher};
use core::borrow::Borrow;

use std::collections::hash_set::HashSet;

use super::super::*;


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

impl<V> Create<V> for HashSet<V>
    where V: Eq + Hash,
{

    #[inline(always)]
    fn create() -> Self { HashSet::<V>::new() }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self { HashSet::<V>::new() }

    #[inline(always)]
    fn add_element(mut self, value: V) -> Self {
        HashSet::<V>::insert(&mut self, value);
        self
    }
}

impl<'a, Q, V> Get<&'a Q> for HashSet<V>
    where Q: Eq + Hash + ?Sized,
          V: Eq + Hash + Borrow<Q>,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, q: &Q) -> Option<&Self::Output> {
        HashSet::get(self, q)
    }
}
