use super::collection_mut::CollectionMut;


pub trait RemoveMut<K: ?Sized>: CollectionMut {
    type Output: ?Sized;

    fn remove(&mut self, key: K) -> Self::Output;
}
