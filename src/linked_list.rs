use collections::linked_list::{self, LinkedList};

use super::*;


impl<T> CollectionMut for LinkedList<T> {
    #[inline(always)]
    fn len(&self) -> usize { self.len() }
    #[inline(always)]
    fn clear(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> DequeMut<T> for LinkedList<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        self.push_front(element)
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        self.push_back(element)
    }
    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        self.pop_back()
    }
    #[inline(always)]
    fn front(&self) -> Option<&T> {
        self.front()
    }
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        self.back()
    }
    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }
}

impl<T> StackMut<T> for LinkedList<T> {
    #[inline(always)]
    fn push(&mut self, element: T) { self.push_back(element) }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> { self.pop_back() }
    #[inline(always)]
    fn top(&self) -> Option<&T> { self.back() }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> { self.back_mut() }
}

impl<T> QueueMut<T> for LinkedList<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) { self.push_back(element) }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> { self.pop_front() }
    #[inline(always)]
    fn peek(&self) -> Option<&T> { self.front() }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> { self.front_mut() }
}

impl<'a, T: 'a> Iterable<'a, &'a T> for LinkedList<T> {
    type Iter = linked_list::Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.iter()
    }
}

impl<'a, T: 'a> IterableMut<'a, &'a mut T> for LinkedList<T> {
    type IterMut = linked_list::IterMut<'a, T>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.iter_mut()
    }
}
