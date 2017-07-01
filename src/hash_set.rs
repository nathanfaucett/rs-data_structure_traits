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
