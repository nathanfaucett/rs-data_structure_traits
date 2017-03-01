use core::ops::Index;

use super::collection::Collection;
use super::iterable::Iterable;


pub trait Seq<'a, Value: 'a>:
    Collection +
    Index<usize, Output = Value> +
    Iterable<'a, &'a Value> {}
