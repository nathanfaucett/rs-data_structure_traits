use alloc::vec::Vec;

use super::super::*;


impl<T> Collection for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        Vec::<T>::len(self)
    }
}

impl<T> CollectionMut for Vec<T> {
    #[inline(always)]
    fn clear(&mut self) {
        Vec::<T>::truncate(self, 0);
    }
}

impl<T> Create<T> for Vec<T> {

    #[inline(always)]
    fn create() -> Self { Vec::<T>::new() }
    #[inline(always)]
    fn create_with_capacity(capacity: usize) -> Self { Vec::<T>::with_capacity(capacity) }

    #[inline(always)]
    fn add_element(&mut self, value: T) {
        Vec::<T>::push(self, value);
    }
}

impl<T> DequeMut<T> for Vec<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        Vec::<T>::insert(self, 0, element)
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        Vec::<T>::push(self, element)
    }

    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        if Vec::<T>::len(self) == 0 {
            None
        } else {
            Some(Vec::<T>::remove(self, 0))
        }
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        Vec::<T>::pop(self)
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
        Vec::<T>::insert(self, index, element)
    }
}

impl<T> RemoveMut<usize> for Vec<T> {
    type Output = T;

    #[inline(always)]
    fn remove(&mut self, index: usize) -> Self::Output {
        Vec::<T>::remove(self, index)
    }
}

impl<T> StackMut<T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, element: T) { Vec::<T>::push_front(self, element) }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> { Vec::<T>::pop_front(self) }
    #[inline(always)]
    fn top(&self) -> Option<&T> { Vec::<T>::front(self) }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> { Vec::<T>::front_mut(self) }
}

impl<T> QueueMut<T> for Vec<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) { Vec::<T>::push_back(self, element) }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> { Vec::<T>::pop_front(self) }
    #[inline(always)]
    fn peek(&self) -> Option<&T> { Vec::<T>::front(self) }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> { Vec::<T>::front_mut(self) }
}

impl<T> Get<usize> for Vec<T> {
    type Output = T;

    #[inline(always)]
    fn get(&self, index: usize) -> Option<&Self::Output> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
}
impl<T> GetMut<usize> for Vec<T> {
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }
}
