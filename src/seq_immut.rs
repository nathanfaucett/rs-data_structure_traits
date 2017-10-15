use super::{Seq, CollectionMut, GetMut, InsertImmut, RemoveImmut};


pub trait SeqImmut<'a, V: 'a>:
    Seq<'a, V> +
    CollectionMut +
    GetMut<usize, Output = V> +
    InsertImmut<usize, V> +
    RemoveImmut<usize> +

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          &'a mut Self: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized,
{}


impl<'a, V, T> SeqImmut<'a, V> for T
    where T: 'a +
            Seq<'a, V> +
            CollectionMut +
            GetMut<usize, Output = V> +
            InsertImmut<usize, V> +
            RemoveImmut<usize>,
          &'a T: 'a + IntoIterator<Item = &'a V>,
          &'a mut T: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized,
{}
