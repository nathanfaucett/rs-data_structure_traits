use super::collection_mut::CollectionMut;
use super::iterable_mut::IterableMut;
use super::get_mut::GetMut;
use super::seq::Seq;


pub trait SeqMut<'a, V: 'a>:
    Seq<'a, V> +
    CollectionMut +
    GetMut<usize, Output = V> +
    IterableMut<'a, &'a mut V> {}
