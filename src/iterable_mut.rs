

pub trait IterableMut<'a> {
    type Output;
    
    fn iter_mut(&'a mut self) -> Self::Output;
}
