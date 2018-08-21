macro_rules! impl_hash_set {
    ($HashSet:ident) => {
        use core::borrow::Borrow;
        use core::hash::{BuildHasher, Hash};

        use super::super::super::*;

        impl<V, S> Collection for $HashSet<V, S>
        where
            V: Eq + Hash,
            S: BuildHasher,
        {
            #[inline(always)]
            fn len(&self) -> usize {
                $HashSet::<V, S>::len(self)
            }
        }

        impl<V, S> CollectionMut for $HashSet<V, S>
        where
            V: Eq + Hash,
            S: BuildHasher,
        {
            #[inline(always)]
            fn clear(&mut self) {
                $HashSet::<V, S>::drain(self);
            }
        }

        impl<V, S> Create<V> for $HashSet<V, S>
        where
            V: Eq + Hash,
            S: Default + BuildHasher,
        {
            #[inline(always)]
            fn create() -> Self {
                $HashSet::<V, S>::default()
            }
            #[inline(always)]
            fn create_with_capacity(_: usize) -> Self {
                $HashSet::<V, S>::default()
            }

            #[inline(always)]
            fn add_element(mut self, value: V) -> Self {
                $HashSet::<V, S>::insert(&mut self, value);
                self
            }
        }

        impl<'a, Q, V> Get<&'a Q> for $HashSet<V>
        where
            Q: Eq + Hash + ?Sized,
            V: Eq + Hash + Borrow<Q>,
        {
            type Output = V;

            #[inline(always)]
            fn get(&self, q: &Q) -> Option<&Self::Output> {
                $HashSet::get(self, q)
            }
        }
    };
}

#[cfg(feature = "hashmap_core")]
mod __impl_hash_set {
    use hashmap_core::HashSet;
    impl_hash_set!(HashSet);
}

#[cfg(feature = "std")]
mod __impl_hash_set {
    use std::collections::hash_set::HashSet;
    impl_hash_set!(HashSet);
}
