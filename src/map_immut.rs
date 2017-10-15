use core::borrow::Borrow;

use super::{Collection, Get, InsertImmut, RemoveImmut};


pub trait MapImmut<'a, K, Q, V>:
    Collection +
    Get<&'a Q, Output = V> +
    InsertImmut<K, V> +
    RemoveImmut<K> +

    where &'a Self: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          &'a mut Self: 'a + IntoIterator<Item = (&'a K, &'a mut V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + Sized,
{}


impl<'a, K, Q, V, T> MapImmut<'a, K, Q, V> for T
    where T: 'a +
            Collection +
            Get<&'a Q, Output = V> +
            InsertImmut<K, V> +
            RemoveImmut<K>,
          &'a T: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          &'a mut T: 'a + IntoIterator<Item = (&'a K, &'a mut V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + Sized,
{}
