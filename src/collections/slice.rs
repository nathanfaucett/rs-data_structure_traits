use core::slice;
use core::slice::SliceExt;

use super::super::*;


impl<T> Collection for [T] {
    #[inline(always)]
    fn len(&self) -> usize {
        SliceExt::len(self)
    }
}

impl<T> Get<usize> for [T] {
    type Output = T;

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Output> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
}
impl<T> GetMut<usize> for [T] {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }
}

impl<'a, T: 'a> Iterable<'a, &'a T> for [T] {
    type Iter = slice::Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T: 'a> IterableMut<'a, &'a mut T> for [T] {
    type IterMut = slice::IterMut<'a, T>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.iter_mut()
    }
}
