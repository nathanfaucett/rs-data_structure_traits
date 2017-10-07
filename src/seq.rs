use super::collection::Collection;
use super::iterable::Iterable;
use super::get::Get;


pub trait Seq<'a, V: 'a>:
    Collection +
    Get<usize, Output = V> +
    Iterable<'a, &'a V> {}
