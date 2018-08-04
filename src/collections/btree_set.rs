use alloc::collections::BTreeSet;

use core::borrow::Borrow;

use super::super::*;

impl<V> Collection for BTreeSet<V>
where
    V: Eq + Ord,
{
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeSet::<V>::len(self)
    }
}

impl<V> CollectionMut for BTreeSet<V>
where
    V: Eq + Ord,
{
    #[inline(always)]
    fn clear(&mut self) {
        BTreeSet::<V>::clear(self);
    }
}

impl<V> Create<V> for BTreeSet<V>
where
    V: Eq + Ord,
{
    #[inline(always)]
    fn create() -> Self {
        BTreeSet::<V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self {
        BTreeSet::<V>::new()
    }

    #[inline(always)]
    fn add_element(mut self, value: V) -> Self {
        BTreeSet::<V>::insert(&mut self, value);
        self
    }
}

impl<'a, Q, V> Get<&'a Q> for BTreeSet<V>
where
    Q: Eq + Ord + ?Sized,
    V: Eq + Ord + Borrow<Q>,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, q: &Q) -> Option<&Self::Output> {
        BTreeSet::get(self, q)
    }
}
