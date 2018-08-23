pub trait RemoveImmut<K>
where
    K: ?Sized,
{
    fn remove(&self, K) -> Self;
}
