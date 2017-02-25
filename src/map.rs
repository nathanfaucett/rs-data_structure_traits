use core::ops::Index;

use super::collection::Collection;


pub trait Map<'a, Key: 'a, Value>:
    Collection +
    Index<&'a Key> {
    fn contains_key(&self, key: &'a Key) -> bool;
}
