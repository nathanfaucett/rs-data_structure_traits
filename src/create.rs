

pub trait Create<T>
    where T: ?Sized,
{
    fn create() -> Self;
    fn create_with_capacity(usize) -> Self;
    fn add_element(&mut self, T);
}
