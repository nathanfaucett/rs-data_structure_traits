use alloc::btree_map::{BTreeMap, Iter, IterMut};

use core::borrow::Borrow;

use super::super::*;


impl<K, V> Collection for BTreeMap<K, V>
    where K: Eq + Ord,
{
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeMap::<K, V>::len(self)
    }
}

impl<K, V> CollectionMut for BTreeMap<K, V>
    where K: Eq + Ord,
{
    #[inline(always)]
    fn clear(&mut self) {
        BTreeMap::<K, V>::clear(self);
    }
}

impl<K, V> Create<(K, V)> for BTreeMap<K, V>
    where K: Eq + Ord,
{
    #[inline(always)]
    fn create() -> Self { BTreeMap::<K, V>::new() }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self { BTreeMap::<K, V>::new() }

    #[inline(always)]
    fn add_element(&mut self, (key, value): (K, V)) {
        BTreeMap::<K, V>::insert(self, key, value);
    }
}

impl<'a, K, V> InsertMut<K, V> for BTreeMap<K, V>
    where K: Eq + Ord,
{
    type Output = Option<V>;

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Self::Output {
        BTreeMap::<K, V>::insert(self, k, v)
    }
}

impl<'a, K, Q: ?Sized, V> RemoveMut<&'a Q> for BTreeMap<K, V>
    where K: Eq + Ord + Borrow<Q>,
          Q: Eq + Ord,
{
    type Output = Option<V>;

    #[inline]
    fn remove(&mut self, k: &Q) -> Self::Output {
        BTreeMap::<K, V>::remove(self, k)
    }
}

impl<'a, K, Q: ?Sized, V> Get<&'a Q> for BTreeMap<K, V>
    where K: Eq + Ord + Borrow<Q>,
          Q: Eq + Ord,
{
    type Output = V;

    #[inline(always)]
    fn get(&self, k: &Q) -> Option<&Self::Output> {
        BTreeMap::get(self, k)
    }
}
impl<'a, K, Q: ?Sized, V> GetMut<&'a Q> for BTreeMap<K, V>
    where K: Eq + Ord + Borrow<Q>,
          Q: Eq + Ord,
{
    #[inline(always)]
    fn get_mut(&mut self, k: &Q) -> Option<&mut Self::Output> {
        BTreeMap::get_mut(self, k)
    }
}

impl<'a, K, V> Iterable<'a, (&'a K, &'a V)> for BTreeMap<K, V>
    where K: 'a + Eq + Ord,
          V: 'a,
{
    type Iter = Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BTreeMap::<K, V>::iter(self)
    }
}

impl<'a, K, V> IterableMut<'a, (&'a K, &'a mut V)> for BTreeMap<K, V>
    where K: 'a + Eq + Ord,
          V: 'a,
{
    type IterMut = IterMut<'a, K, V>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        BTreeMap::<K, V>::iter_mut(self)
    }
}

impl<'a, K, Q: ?Sized, V> MapMut<'a, K, Q, V> for BTreeMap<K, V>
    where K: 'a + Eq + Ord + Borrow<Q>,
          Q: 'a + Eq + Ord,
          V: 'a,
{}

impl<'a, K, Q: ?Sized, V> Map<'a, K, Q, V> for BTreeMap<K, V>
    where K: 'a + Eq + Ord + Borrow<Q>,
          Q: 'a + Eq + Ord,
          V: 'a,
{
    #[inline(always)]
    fn contains_key(&self, k: &Q) -> bool {
        BTreeMap::contains_key(self, k)
    }
}
