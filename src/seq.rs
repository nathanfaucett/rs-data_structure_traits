use core::ops::Index;

use super::collection::Collection;
use super::insert::Insert;
use super::remove::Remove;
use super::iterable::Iterable;


pub trait Seq<'a, Value: 'a>:
    Collection +

    Index<usize, Output = Value> +

    Insert<usize, Value> +
    Remove<usize> +

    Iterable<'a, &'a Value> {}
