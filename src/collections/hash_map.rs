macro_rules! impl_hash_map {
    ($HashMap:ident) => {
        use core::borrow::Borrow;
        use core::hash::{BuildHasher, Hash};

        use super::super::super::*;

        impl<K, V, S> Collection for $HashMap<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher,
        {
            #[inline(always)]
            fn len(&self) -> usize {
                $HashMap::<K, V, S>::len(self)
            }
        }

        impl<K, V, S> CollectionMut for $HashMap<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher,
        {
            #[inline(always)]
            fn clear(&mut self) {
                $HashMap::<K, V, S>::drain(self);
            }
        }

        impl<K, V, S> Create<(K, V)> for $HashMap<K, V, S>
        where
            K: Eq + Hash,
            S: Default + BuildHasher,
        {
            #[inline(always)]
            fn create() -> Self {
                $HashMap::<K, V, S>::default()
            }
            #[inline(always)]
            fn create_with_capacity(_: usize) -> Self {
                $HashMap::<K, V, S>::default()
            }

            #[inline(always)]
            fn add_element(mut self, (key, value): (K, V)) -> Self {
                $HashMap::<K, V, S>::insert(&mut self, key, value);
                self
            }
        }

        impl<'a, K, V, S> Insert<K, V> for $HashMap<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher,
        {
            type Output = Option<V>;

            #[inline]
            fn insert(&mut self, k: K, v: V) -> Self::Output {
                $HashMap::<K, V, S>::insert(self, k, v)
            }
        }

        impl<K, V, S> Add<(K, V)> for $HashMap<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher,
        {
            type Output = Option<V>;

            #[inline(always)]
            fn add(&mut self, (k, v): (K, V)) -> Self::Output {
                $HashMap::<K, V, S>::insert(self, k, v)
            }
        }

        impl<'a, K, Q: ?Sized, V, S> Remove<&'a Q> for $HashMap<K, V, S>
        where
            K: Eq + Hash + Borrow<Q>,
            Q: Eq + Hash,
            S: BuildHasher,
        {
            type Output = Option<V>;

            #[inline]
            fn remove(&mut self, q: &Q) -> Self::Output {
                $HashMap::<K, V, S>::remove(self, q)
            }
        }

        impl<'a, K, Q: ?Sized, V, S> Get<&'a Q> for $HashMap<K, V, S>
        where
            K: Eq + Hash + Borrow<Q>,
            Q: Eq + Hash,
            S: BuildHasher,
        {
            type Output = V;

            #[inline(always)]
            fn get(&self, q: &Q) -> Option<&Self::Output> {
                $HashMap::<K, V, S>::get(self, q)
            }
        }
        impl<'a, K, Q: ?Sized, V, S> GetMut<&'a Q> for $HashMap<K, V, S>
        where
            K: Eq + Hash + Borrow<Q>,
            Q: Eq + Hash,
            S: BuildHasher,
        {
            #[inline(always)]
            fn get_mut(&mut self, q: &Q) -> Option<&mut Self::Output> {
                $HashMap::<K, V, S>::get_mut(self, q)
            }
        }
    };
}

#[cfg(feature = "hashmap_core")]
mod __impl_hash_map {
    use hashmap_core::HashMap;
    impl_hash_map!(HashMap);
}

#[cfg(feature = "std")]
mod __impl_hash_map {
    use std::collections::hash_map::HashMap;
    impl_hash_map!(HashMap);
}
