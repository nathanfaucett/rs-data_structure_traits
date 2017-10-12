use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::set::Set;


pub trait SetMut<'a, V, Q>:
    Set<'a, V, Q> +
    CollectionMut

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          &'a mut Self: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}
