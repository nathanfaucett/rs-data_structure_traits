use core::ops::IndexMut;

use super::insert::Insert;
use super::iterable_mut::IterableMut;
use super::map::Map;
use super::remove::Remove;


pub trait MapMut<'a, Key: 'a, Value: 'a>:
    Map<'a, Key, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a Key, Output = Value> +
    IndexMut<&'a Key, Output = Value> +
    IterableMut<'a, (&'a Key, &'a mut Value)> {}
