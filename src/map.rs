use core::ops::Index;
use core::borrow::Borrow;

use super::collection::Collection;
use super::insert::Insert;
use super::remove::Remove;
use super::iterable::Iterable;


pub trait Map<'a, Key, BorrowKey, Value>:
    Collection +

    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a BorrowKey> +

    Index<&'a BorrowKey, Output = Value> +

    Iterable<'a, (&'a Key, &'a Value)>

    where Key: 'a + Borrow<BorrowKey>,
          BorrowKey: 'a + ?Sized,
          Value: 'a
{
    fn contains_key(&self, k: &BorrowKey) -> bool;
}
