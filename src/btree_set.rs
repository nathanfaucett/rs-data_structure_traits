use alloc::btree_set::BTreeSet;

use super::*;


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

impl<V> Create for BTreeSet<V>
    where V: Eq + Ord,
{
    #[inline(always)]
    fn create() -> Self {
        BTreeSet::<V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self {
        BTreeSet::<V>::new()
    }
}

impl<V> AddElementMut<V> for BTreeSet<V>
    where V: Eq + Ord,
{
    #[inline(always)]
    fn add_element(&mut self, element: V) {
        BTreeSet::<V>::insert(self, element);
    }
}
