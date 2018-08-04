use alloc::collections::BTreeMap;

use core::borrow::Borrow;

use super::super::*;

impl<K, V> Collection for BTreeMap<K, V>
where
    K: Eq + Ord,
{
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeMap::<K, V>::len(self)
    }
}

impl<K, V> CollectionMut for BTreeMap<K, V>
where
    K: Eq + Ord,
{
    #[inline(always)]
    fn clear(&mut self) {
        BTreeMap::<K, V>::clear(self);
    }
}

impl<K, V> Create<(K, V)> for BTreeMap<K, V>
where
    K: Eq + Ord,
{
    #[inline(always)]
    fn create() -> Self {
        BTreeMap::<K, V>::new()
    }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self {
        BTreeMap::<K, V>::new()
    }

    #[inline(always)]
    fn add_element(mut self, (key, value): (K, V)) -> Self {
        BTreeMap::<K, V>::insert(&mut self, key, value);
        self
    }
}

impl<'a, K, V> Insert<K, V> for BTreeMap<K, V>
where
    K: Eq + Ord,
{
    type Output = Option<V>;

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Self::Output {
        BTreeMap::<K, V>::insert(self, k, v)
    }
}

impl<'a, K, Q: ?Sized, V> Remove<&'a Q> for BTreeMap<K, V>
where
    K: Eq + Ord + Borrow<Q>,
    Q: Eq + Ord,
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, q: &Q) -> Self::Output {
        BTreeMap::<K, V>::remove(self, q)
    }
}

impl<'a, K, Q: ?Sized, V> Get<&'a Q> for BTreeMap<K, V>
where
    K: Eq + Ord + Borrow<Q>,
    Q: Eq + Ord,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, q: &Q) -> Option<&Self::Output> {
        BTreeMap::<K, V>::get(self, q)
    }
}
impl<'a, K, Q: ?Sized, V> GetMut<&'a Q> for BTreeMap<K, V>
where
    K: Eq + Ord + Borrow<Q>,
    Q: Eq + Ord,
{
    #[inline(always)]
    fn get_mut(&mut self, q: &Q) -> Option<&mut Self::Output> {
        BTreeMap::<K, V>::get_mut(self, q)
    }
}
