use core::borrow::Borrow;

use super::{CollectionMut, GetMut, Set};

pub trait SetMut<'a, V, Q>: Set<'a, V, Q> + CollectionMut
where
    V: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
{
}

impl<'a, V, Q, T> SetMut<'a, V, Q> for T
where
    T: 'a + Set<'a, V, Q> + CollectionMut + GetMut<&'a Q, Output = V>,
    V: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
{}
