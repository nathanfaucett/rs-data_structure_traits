use super::{Collection, Get};

pub trait Seq<'a, V: 'a>: Collection + Get<usize, Output = V>
where
    V: 'a + ?Sized,
{
}

impl<'a, V, T> Seq<'a, V> for T
where
    T: 'a + Collection + Get<usize, Output = V>,
    V: 'a + ?Sized,
{}
