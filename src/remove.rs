

pub trait Remove<Key: ?Sized> {
    type Output: ?Sized;

    fn remove(&mut self, key: Key) -> Self::Output;
}
