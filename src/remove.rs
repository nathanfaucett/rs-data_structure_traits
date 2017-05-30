use super::collection::Collection;


pub trait Remove<Key: ?Sized>: Collection {
    fn remove(&self, key: Key) -> Self;
}
