use collections::vec::Vec;

use core::slice;

use super::*;


impl<T> Collection for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionMut for Vec<T> {
    #[inline(always)]
    fn clear(&mut self) {
        self.truncate(0);
    }
}

impl<T> DequeMut<T> for Vec<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        self.insert(0, element)
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        self.push(element)
    }

    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.remove(0))
        }
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        self.pop()
    }

    #[inline(always)]
    fn front(&self) -> Option<&T> {
        self.first()
    }
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        self.last()
    }

    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        self.first_mut()
    }
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.last_mut()
    }
}

impl<T> InsertMut<usize, T> for Vec<T> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, element: T) -> Self::Output {
        self.insert(index, element)
    }
}

impl<T> RemoveMut<usize> for Vec<T> {
    type Output = T;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        self.remove(index)
    }
}

impl<T> StackMut<T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, element: T) { self.push_front(element) }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> { self.pop_front() }
    #[inline(always)]
    fn top(&self) -> Option<&T> { self.front() }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<T> QueueMut<T> for Vec<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) { self.push_back(element) }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> { self.pop_front() }
    #[inline(always)]
    fn peek(&self) -> Option<&T> { self.front() }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<'a, T: 'a> Iterable<'a, &'a T> for Vec<T> {
    type Iter = slice::Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        (&**self).iter()
    }
}

impl<'a, T: 'a> IterableMut<'a, &'a mut T> for Vec<T> {
    type IterMut = slice::IterMut<'a, T>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        (&mut **self).iter_mut()
    }
}

impl<'a, T: 'a> SeqMut<'a, T> for Vec<T> {}
