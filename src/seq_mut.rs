use super::{CollectionMut, GetMut, InsertMut, RemoveMut, Seq};

pub trait SeqMut<'a, V: 'a>:
    Seq<'a, V>
    + CollectionMut
    + GetMut<usize, Output = V>
    + InsertMut<usize, V, Output = ()>
    + RemoveMut<usize, Output = V>
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
        + InsertMut<usize, V, Output = ()>
        + RemoveMut<usize, Output = V>,
    V: 'a + ?Sized,
{}
