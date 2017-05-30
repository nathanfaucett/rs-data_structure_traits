use core::ops::{Index, IndexMut};

use super::collection_mut::CollectionMut;
use super::insert_mut::InsertMut;
use super::remove_mut::RemoveMut;
use super::iterable::Iterable;
use super::iterable_mut::IterableMut;


pub trait SeqMut<'a, Value: 'a>:
    CollectionMut +

    Index<usize, Output = Value> +
    IndexMut<usize, Output = Value> +

    InsertMut<usize, Value> +
    RemoveMut<usize, Output = Value> +

    Iterable<'a, &'a Value> +
    IterableMut<'a, &'a mut Value> {}
