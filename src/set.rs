use core::borrow::Borrow;

use super::{Collection, Get};


pub trait Set<'a, V, Q>:
    Collection +
    Get<&'a Q, Output = V> +

    where &'a Self: 'a + IntoIterator<Item = &'a V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{
    #[inline(always)]
    fn contains(&self, q: &'a Q) -> bool {
        self.get(q).is_some()
    }
}


impl<'a, V, Q, T> Set<'a, V, Q> for T
    where T: 'a + Collection + Get<&'a Q, Output = V>,
          &'a T: 'a + IntoIterator<Item = &'a V>,
          V: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
{}
