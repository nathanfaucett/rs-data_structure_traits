use core::ops::Index;

use super::collection::Collection;


pub trait Seq<Value>:
    Collection +
    Index<usize, Output = Value> {}
