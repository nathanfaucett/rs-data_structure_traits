pub trait AddImmut<V>
where
    V: ?Sized,
{
    fn add(&self, V) -> Self;
}
