use super::{Insert, Remove, Seq};

pub trait SeqImmut<'a, V: 'a>:
    Seq<'a, V> + Insert<usize, V, Output = ()> + Remove<usize, Output = V>
where
    V: 'a + ?Sized,
{
}

impl<'a, V, T> SeqImmut<'a, V> for T
where
    T: 'a + Seq<'a, V> + Insert<usize, V, Output = ()> + Remove<usize, Output = V>,
    V: 'a + ?Sized,
{}
