use super::collection_mut::CollectionMut;


pub trait InsertMut<K, V>: CollectionMut {
    type Output;

    fn insert(&mut self, key: K, element: V) -> Self::Output;
}
