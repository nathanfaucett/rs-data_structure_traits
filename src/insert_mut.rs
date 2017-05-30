use super::collection_mut::CollectionMut;


pub trait InsertMut<Key, Value>: CollectionMut {
    type Output;

    fn insert(&mut self, key: Key, element: Value) -> Self::Output;
}
