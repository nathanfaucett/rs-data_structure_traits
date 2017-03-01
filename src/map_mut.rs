use core::ops::IndexMut;

use super::insert::Insert;
use super::iterable_mut::IterableMut;
use super::map::Map;
use super::remove::Remove;


pub trait MapMut<'a, 'b, Key: 'a, Value: 'b>:
    Map<'a, 'b, Key, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a Key, Output = Value> +
    IndexMut<&'a Key, Output = Value> +
    IterableMut<'a, (&'a mut Key, &'b mut Value)> {}
