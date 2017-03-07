use core::ops::IndexMut;
use core::borrow::Borrow;

use super::insert::Insert;
use super::iterable_mut::IterableMut;
use super::map::Map;
use super::remove::Remove;


pub trait MapMut<'a, Key, BorrowKey, Value>:
    Map<'a, Key, BorrowKey, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a BorrowKey, Output = Option<Value>> +
    IndexMut<&'a BorrowKey, Output = Value> +
    IterableMut<'a, (&'a Key, &'a mut Value)>

    where Key: 'a + Borrow<BorrowKey>,
          BorrowKey: 'a + ?Sized,
          Value: 'a
    {}
