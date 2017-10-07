use core::borrow::Borrow;
use core::hash::{Hash, BuildHasher};

use std::collections::hash_set::{HashSet, Iter};

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
    fn add_element(&mut self, value: V) {
        HashSet::<V>::insert(self, value);
    }
}

impl<'a, V> Iterable<'a, &'a V> for HashSet<V>
    where V: 'a + Eq + Hash,
{
    type Iter = Iter<'a, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        HashSet::<V>::iter(self)
    }
}

impl<'a, V, Q: ?Sized> Set<'a, V, Q> for HashSet<V>
    where V: 'a + Eq + Hash + Borrow<Q>,
          Q: 'a + Eq + Hash,
{
    #[inline(always)]
    fn contains(&self, v: &Q) -> bool {
        HashSet::contains(self, v)
    }
}
