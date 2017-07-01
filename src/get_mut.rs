use super::get::Get;


pub trait GetMut<K>: Get<K>
    where K: ?Sized,
{
    fn get_mut(&mut self, K) -> &mut Self::Output;
}
