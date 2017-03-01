use core::ops::Index;

use super::collection::Collection;
use super::iterable::Iterable;


pub trait Seq<'a, Value>:
    Collection +
    Index<usize, Output = Value> +
    Iterable<'a> {}
