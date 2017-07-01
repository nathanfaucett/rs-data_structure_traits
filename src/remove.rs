use super::collection::Collection;


pub trait Remove<K: ?Sized>: Collection {
    fn remove(&self, key: K) -> Self;
}
