use super::{CollectionMut, InsertImmut, RemoveImmut, Seq};

pub trait SeqImmut<'a, V: 'a>:
    Seq<'a, V> + CollectionMut + InsertImmut<usize, V> + RemoveImmut<usize>
+
where
    &'a Self: 'a + IntoIterator<Item = &'a V>,
    V: 'a + ?Sized,
{
}

impl<'a, V, T> SeqImmut<'a, V> for T
where
    T: 'a + Seq<'a, V> + CollectionMut + InsertImmut<usize, V> + RemoveImmut<usize>,
    &'a T: 'a + IntoIterator<Item = &'a V>,
    V: 'a + ?Sized,
{}
