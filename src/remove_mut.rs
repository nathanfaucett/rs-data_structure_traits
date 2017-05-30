use super::collection_mut::CollectionMut;


pub trait RemoveMut<Key: ?Sized>: CollectionMut {
    type Output: ?Sized;

    fn remove(&mut self, key: Key) -> Self::Output;
}
