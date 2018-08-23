use super::{CollectionMut, GetMut, Insert, Remove, Seq};

pub trait SeqMut<'a, V: 'a>:
    Seq<'a, V>
    + CollectionMut
    + GetMut<usize, Output = V>
    + Insert<usize, V, Output = ()>
    + Remove<usize, Output = V>
where
    V: 'a + ?Sized,
{
}

impl<'a, V, T> SeqMut<'a, V> for T
where
    T: 'a
        + Seq<'a, V>
        + CollectionMut
        + GetMut<usize, Output = V>
        + Insert<usize, V, Output = ()>
        + Remove<usize, Output = V>,
    V: 'a + ?Sized,
{}
