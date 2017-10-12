use super::collection_mut::CollectionMut;
use super::get_mut::GetMut;
use super::seq::Seq;


pub trait SeqMut<'a, V: 'a>:
    Seq<'a, V> +
    CollectionMut +
    GetMut<usize, Output = V> +

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          &'a mut Self: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized,
{}


impl<'a, V, T> SeqMut<'a, V> for T
    where T: 'a + Seq<'a, V> + CollectionMut + GetMut<usize, Output = V>,
          &'a T: 'a + IntoIterator<Item = &'a V>,
          &'a mut T: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized,
{}
