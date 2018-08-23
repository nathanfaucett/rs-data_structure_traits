#[cfg(not(feature = "std"))]
use alloc::vec_deque::VecDeque;
#[cfg(feature = "std")]
use std::collections::VecDeque;

use super::super::*;

impl<T> Collection for VecDeque<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        VecDeque::<T>::len(self)
    }
}

impl<T> CollectionMut for VecDeque<T> {
    #[inline(always)]
    fn clear(&mut self) {
        VecDeque::<T>::truncate(self, 0);
    }
}

impl<T> Create<T> for VecDeque<T> {
    #[inline(always)]
    fn create() -> Self {
        VecDeque::<T>::new()
    }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self {
        VecDeque::<T>::with_capacity(capacity)
    }

    #[inline(always)]
    fn add_element(mut self, value: T) -> Self {
        VecDeque::<T>::push(&mut self, value);
        self
    }
}

impl<T> DequeMut<T> for VecDeque<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        VecDeque::<T>::insert(self, 0, element)
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        VecDeque::<T>::push(self, element)
    }

    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        if VecDeque::<T>::len(self) == 0 {
            None
        } else {
            VecDeque::<T>::remove(self, 0)
        }
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        VecDeque::<T>::pop(self)
    }

    #[inline(always)]
    fn front(&self) -> Option<&T> {
        VecDeque::<T>::front(self)
    }
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        VecDeque::<T>::back(self)
    }

    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        VecDeque::<T>::front_mut(self)
    }
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        VecDeque::<T>::back_mut(self)
    }
}

impl<T> InsertMut<usize, T> for VecDeque<T> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, element: T) -> Self::Output {
        VecDeque::<T>::insert(self, index, element)
    }
}

impl<T> RemoveMut<usize> for VecDeque<T> {
    type Output = Option<T>;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        VecDeque::<T>::remove(self, index)
    }
}

impl<T> StackMut<T> for VecDeque<T> {
    #[inline(always)]
    fn push(&mut self, element: T) {
        VecDeque::<T>::push_front(self, element)
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        VecDeque::<T>::pop_front(self)
    }
    #[inline(always)]
    fn top(&self) -> Option<&T> {
        VecDeque::<T>::front(self)
    }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> {
        VecDeque::<T>::front_mut(self)
    }
}

impl<T> QueueMut<T> for VecDeque<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) {
        VecDeque::<T>::push_back(self, element)
    }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> {
        VecDeque::<T>::pop_front(self)
    }
    #[inline(always)]
    fn peek(&self) -> Option<&T> {
        VecDeque::<T>::front(self)
    }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> {
        VecDeque::<T>::front_mut(self)
    }
}

impl<T> Get<usize> for VecDeque<T> {
    type Output = T;

    #[inline(always)]
    fn get(&self, index: usize) -> Option<&Self::Output> {
        VecDeque::<T>::get(self, index)
    }
}
impl<T> GetMut<usize> for VecDeque<T> {
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        VecDeque::<T>::get_mut(self, index)
    }
}
