use super::collection::Collection;
use super::insert::Insert;
use super::remove::Remove;
use super::iterable::Iterable;
use super::get::Get;


pub trait Seq<'a, V: 'a>:
    Collection +

    Get<usize, Output = V> +

    Insert<usize, V> +
    Remove<usize> +

    Iterable<'a, &'a V> {}
