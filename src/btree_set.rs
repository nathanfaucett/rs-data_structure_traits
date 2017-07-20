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
