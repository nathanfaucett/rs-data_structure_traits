use core::ops::Index;

use super::collection::Collection;
use super::iterable::Iterable;


pub trait Map<'a, Key: 'a, Value: 'a>:
    Collection +
    Index<&'a Key, Output = Value> +
    Iterable<'a, (&'a Key, &'a Value)>
{
    fn contains_key(&self, key: &'a Key) -> bool;
}
