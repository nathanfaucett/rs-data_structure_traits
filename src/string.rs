use collections::string::String;

use core::str::{self, Chars};

use super::*;


impl<'a> Collection for &'a str {
    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl<'a> Iterable<'a, char> for &'a str {
    type Iter = Chars<'a>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        (&**self).chars()
    }
}


impl Collection for String {
    #[inline(always)]
    fn len(&self) -> usize {
        String::len(self)
    }
}

impl CollectionMut for String {
    #[inline(always)]
    fn clear(&mut self) {
        String::clear(self)
    }
}

impl InsertMut<usize, char> for String {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, element: char) -> Self::Output {
        self.insert(index, element)
    }
}

impl RemoveMut<usize> for String {
    type Output = char;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        self.remove(index)
    }
}

impl<'a> Iterable<'a, char> for String {
    type Iter = Chars<'a>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        (&**self).chars()
    }
}
