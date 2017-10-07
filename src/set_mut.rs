use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::set::Set;
use super::iterable_mut::IterableMut;


pub trait SetMut<'a, V, Q>:
    Set<'a, V, Q> +
    CollectionMut +
    IterableMut<'a, &'a mut V>

    where V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{
    fn contains(&self, &Q) -> bool;
}
