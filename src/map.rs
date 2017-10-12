use core::borrow::Borrow;

use super::collection::Collection;
use super::get::Get;


pub trait Map<'a, K, Q, V>:
    Collection +
    Get<&'a Q, Output = V> +

    where &'a Self: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{
    fn contains_key(&self, &'a Q) -> bool;
}


impl<'a, K, Q, V, T> Map<'a, K, Q, V> for T
    where T: 'a + Collection + Get<&'a Q, Output = V>,
          &'a T: 'a + IntoIterator<Item = (&'a K, &'a V)>,
          K: 'a + ?Sized + Borrow<Q>,
          Q: 'a + ?Sized,
          V: 'a + ?Sized,
{
    #[inline(always)]
    fn contains_key(&self, q: &'a Q) -> bool {
        self.get(q).is_some()
    }
}
