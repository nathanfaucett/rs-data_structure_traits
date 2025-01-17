pub trait Insert<K, V>
where
    K: ?Sized,
    V: ?Sized,
{
    type Output: ?Sized;

    fn insert(&mut self, K, V) -> Self::Output;
}
