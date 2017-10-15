use core::borrow::Borrow;

use super::Set;


pub trait SetImmut<'a, V, Q>:
    Set<'a, V, Q> +

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          &'a mut Self: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}


impl<'a, V, Q, T> SetImmut<'a, V, Q> for T
    where T: 'a + Set<'a, V, Q>,
          &'a T: 'a + IntoIterator<Item = &'a V>,
          &'a mut T: 'a + IntoIterator<Item = &'a mut V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}
