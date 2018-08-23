use core::borrow::Borrow;

use super::{Collection, Get};

pub trait Map<'a, K, Q, V>: Collection + Get<&'a Q, Output = V>
where
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + ?Sized,
{
    #[inline(always)]
    fn contains_key(&self, q: &'a Q) -> bool {
        self.get(q).is_some()
    }
}

impl<'a, K, Q, V, T> Map<'a, K, Q, V> for T
where
    T: 'a + Collection + Get<&'a Q, Output = V>,
    K: 'a + ?Sized + Borrow<Q>,
    Q: 'a + ?Sized,
    V: 'a + ?Sized,
{}
