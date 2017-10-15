

pub trait Remove<K>
    where K: ?Sized,
{
    type Output: ?Sized;

    fn remove(&mut self, K) -> Self::Output;
}
