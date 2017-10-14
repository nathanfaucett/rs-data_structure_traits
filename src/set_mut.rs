use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::get_mut::GetMut;
use super::set::Set;


pub trait SetMut<'a, V, Q>:
    Set<'a, V, Q> +
    CollectionMut

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          &'a mut Self: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}


impl<'a, V, Q, T> SetMut<'a, V, Q> for T
    where T: 'a + Set<'a, V, Q> + CollectionMut + GetMut<&'a Q, Output = V>,
          &'a T: 'a + IntoIterator<Item = &'a V>,
          &'a mut T: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}
