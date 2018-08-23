use core::borrow::Borrow;

use super::{AddImmut, RemoveImmut, Set};

pub trait SetImmut<'a, V, Q>: Set<'a, V, Q> + RemoveImmut<&'a Q> + AddImmut<V>
where
    V: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
{
}

impl<'a, V, Q, T> SetImmut<'a, V, Q> for T
where
    T: 'a + SetImmut<'a, V, Q> + RemoveImmut<&'a Q> + AddImmut<V>,
    V: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
{}
