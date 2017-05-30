use super::collection::Collection;


pub trait Insert<Key, Value>: Collection {
    type Output;

    fn insert(&self, key: Key, element: Value) -> Self::Output;
}
