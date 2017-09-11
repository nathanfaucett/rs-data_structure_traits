use alloc::string::String;

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

impl Create for String {
    #[inline(always)]
    fn create() -> Self {
        String::new()
    }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }
}

impl AddElementMut<char> for String {
    #[inline(always)]
    fn add_element(&mut self, element: char) {
        String::push(self, element)
    }
}

impl InsertMut<usize, char> for String {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, element: char) -> Self::Output {
        String::insert(self, index, element)
    }
}

impl RemoveMut<usize> for String {
    type Output = char;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        String::remove(self, index)
    }
}

impl<'a> Iterable<'a, char> for String {
    type Iter = Chars<'a>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        (&**self).chars()
    }
}
