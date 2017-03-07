use core::ops::Index;

use super::collection::Collection;
use super::iterable::Iterable;


pub trait Map<'a, 'b, Key: 'a, Value: 'b>:
    Collection +
    Index<&'a Key, Output = Value> +
    Iterable<'a, (&'a Key, &'b Value)>
{
    fn contains_key(&self, key: &'a Key) -> bool;
}
