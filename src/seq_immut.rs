use super::{InsertImmut, RemoveImmut, Seq};

pub trait SeqImmut<'a, V: 'a>: Seq<'a, V> + InsertImmut<usize, V> + RemoveImmut<usize>
where
    V: 'a + ?Sized,
{
}

impl<'a, V, T> SeqImmut<'a, V> for T
where
    T: 'a + Seq<'a, V> + InsertImmut<usize, V> + RemoveImmut<usize>,
    V: 'a + ?Sized,
{}
