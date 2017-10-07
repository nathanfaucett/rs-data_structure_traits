use alloc::btree_set::{BTreeSet, Iter};
use core::borrow::Borrow;

use super::super::*;


impl<V> Collection for BTreeSet<V>
    where V: Eq + Ord,
{
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeSet::<V>::len(self)
    }
}

impl<V> CollectionMut for BTreeSet<V>
    where V: Eq + Ord,
{
    #[inline(always)]
    fn clear(&mut self) {
        BTreeSet::<V>::clear(self);
    }
}

impl<V> Create<V> for BTreeSet<V>
    where V: Eq + Ord,
{

    #[inline(always)]
    fn create() -> Self { BTreeSet::<V>::new() }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self { BTreeSet::<V>::new() }

    #[inline(always)]
    fn add_element(&mut self, value: V) {
        BTreeSet::<V>::insert(self, value);
    }
}

impl<'a, V> Iterable<'a, &'a V> for BTreeSet<V>
    where V: 'a + Eq + Ord,
{
    type Iter = Iter<'a, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BTreeSet::<V>::iter(self)
    }
}

impl<'a, V, Q: ?Sized> Set<'a, V, Q> for BTreeSet<V>
    where V: 'a + Eq + Ord + Borrow<Q>,
          Q: 'a + Eq + Ord,
{
    #[inline(always)]
    fn contains(&self, v: &Q) -> bool {
        BTreeSet::contains(self, v)
    }
}
