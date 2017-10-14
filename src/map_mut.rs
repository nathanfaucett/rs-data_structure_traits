use core::borrow::Borrow;

use super::collection_mut::CollectionMut;
use super::get_mut::GetMut;
use super::map::Map;


pub trait MapMut<'a, K, Q, V>:
    Map<'a, K, Q, V> +
    CollectionMut +
    GetMut<&'a Q, Output = V> +

    where &'a Self: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          &'a mut Self: 'a + IntoIterator<Item = (&'a K, &'a mut V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{}


impl<'a, K, Q, V, T> MapMut<'a, K, Q, V> for T
    where T: 'a + Map<'a, K, Q, V> + CollectionMut + GetMut<&'a Q, Output = V>,
          &'a T: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          &'a mut T: 'a + IntoIterator<Item = (&'a K, &'a mut V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{}
