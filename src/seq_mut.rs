use super::collection_mut::CollectionMut;
use super::insert_mut::InsertMut;
use super::remove_mut::RemoveMut;
use super::iterable::Iterable;
use super::iterable_mut::IterableMut;
use super::get::Get;
use super::get_mut::GetMut;


pub trait SeqMut<'a, V: 'a>:
    CollectionMut +

    Get<usize, Output = V> +
    GetMut<usize, Output = V> +

    InsertMut<usize, V> +
    RemoveMut<usize, Output = V> +

    Iterable<'a, &'a V> +
    IterableMut<'a, &'a mut V> {}
