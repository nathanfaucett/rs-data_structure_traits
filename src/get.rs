

pub trait Get<K>
    where K: ?Sized,
{
    type Output: ?Sized;

    fn get(&self, K) -> &Self::Output;
}
