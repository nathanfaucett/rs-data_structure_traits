use core::ops::IndexMut;

use super::map::Map;
use super::insert::Insert;
use super::remove::Remove;


pub trait MapMut<'a, Key: 'a, Value>:
    Map<'a, Key, Value> +
    Insert<Key, Value, Output = Option<Value>> +
    Remove<&'a Key, Output = Value> +
    IndexMut<&'a Key, Output = Value> {}
