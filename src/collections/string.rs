#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

use core::str;

use super::super::*;

impl<'a> Collection for &'a str {
    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
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

impl Create<char> for String {
    #[inline(always)]
    fn create() -> Self {
        String::new()
    }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }

    #[inline(always)]
    fn add_element(mut self, ch: char) -> Self {
        String::push(&mut self, ch);
        self
    }
}

impl Insert<usize, char> for String {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, element: char) -> Self::Output {
        String::insert(self, index, element)
    }
}

impl Remove<usize> for String {
    type Output = char;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        String::remove(self, index)
    }
}
