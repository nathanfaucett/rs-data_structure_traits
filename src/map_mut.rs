use core::ops::{Index, IndexMut};
use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::insert_mut::InsertMut;
use super::remove_mut::RemoveMut;
use super::iterable::Iterable;
use super::iterable_mut::IterableMut;


pub trait MapMut<'a, Key, BorrowKey, Value>:
    CollectionMut +

    InsertMut<Key, Value, Output = Option<Value>> +
    RemoveMut<&'a BorrowKey, Output = Option<Value>> +

    Index<&'a BorrowKey, Output = Value> +
    IndexMut<&'a BorrowKey, Output = Value> +

    Iterable<'a, (&'a Key, &'a Value)> +
    IterableMut<'a, (&'a Key, &'a mut Value)>

    where Key: 'a + Borrow<BorrowKey>,
          BorrowKey: 'a + ?Sized,
          Value: 'a
{
    fn contains_key(&self, k: &BorrowKey) -> bool;
}
