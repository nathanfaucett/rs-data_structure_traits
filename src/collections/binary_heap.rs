#[cfg(not(feature = "std"))]
use alloc::collections::BinaryHeap;
#[cfg(feature = "std")]
use std::collections::BinaryHeap;

use super::super::*;

impl<V> Collection for BinaryHeap<V>
where
    V: Ord,
{
    #[inline(always)]
    fn len(&self) -> usize {
        BinaryHeap::<V>::len(self)
    }
}

impl<V> CollectionMut for BinaryHeap<V>
where
    V: Ord,
{
    #[inline(always)]
    fn clear(&mut self) {
        BinaryHeap::<V>::clear(self);
    }
}

impl<V> Create<V> for BinaryHeap<V>
where
    V: Ord,
{
    #[inline(always)]
    fn create() -> Self {
        BinaryHeap::<V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self {
        BinaryHeap::<V>::new()
    }

    #[inline(always)]
    fn add_element(mut self, value: V) -> Self {
        BinaryHeap::<V>::push(&mut self, value);
        self
    }
}

impl<V> Add<V> for BinaryHeap<V>
where
    V: Ord,
{
    type Output = ();

    #[inline(always)]
    fn add(&mut self, v: V) -> Self::Output {
        BinaryHeap::<V>::push(self, v)
    }
}
