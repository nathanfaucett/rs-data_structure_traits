pub trait Remove<K>
where
    K: ?Sized,
{
    fn remove(&self, K) -> Self;
}
