

pub trait Iterable<'a> {
    type Output;
    
    fn iter(&'a self) -> Self::Output;
}
