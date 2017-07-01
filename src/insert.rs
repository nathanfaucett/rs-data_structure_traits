use super::collection::Collection;


pub trait Insert<K, V>: Collection {
    type Output;

    fn insert(&self, key: K, element: V) -> Self::Output;
}
