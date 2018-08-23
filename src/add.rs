pub trait Add<V>
where
    V: ?Sized,
{
    type Output: ?Sized;

    fn add(&mut self, V) -> Self::Output;
}
