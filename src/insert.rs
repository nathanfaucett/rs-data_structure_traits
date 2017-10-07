

pub trait Insert<K, V>
    where K: ?Sized,
          V: ?Sized,
{
    fn insert(&self, K, V) -> Self;
}
