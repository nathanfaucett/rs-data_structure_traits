use alloc::collections::LinkedList;

use super::super::*;

impl<T> Collection for LinkedList<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        LinkedList::<T>::len(self)
    }
}

impl<T> CollectionMut for LinkedList<T> {
    fn clear(&mut self) {
        while let Some(_) = LinkedList::<T>::pop_front(self) {}
    }
}

impl<T> Create<T> for LinkedList<T> {
    #[inline(always)]
    fn create() -> Self {
        LinkedList::<T>::new()
    }
    #[inline(always)]
    fn create_with_capacity(_: usize) -> Self {
        LinkedList::<T>::new()
    }

    #[inline(always)]
    fn add_element(mut self, value: T) -> Self {
        LinkedList::<T>::push(&mut self, value);
        self
    }
}

impl<T> Deque<T> for LinkedList<T> {
    #[inline(always)]
    fn push_front(&mut self, element: T) {
        LinkedList::<T>::push_front(self, element)
    }
    #[inline(always)]
    fn push_back(&mut self, element: T) {
        LinkedList::<T>::push_back(self, element)
    }
    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        LinkedList::<T>::pop_front(self)
    }
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        LinkedList::<T>::pop_back(self)
    }
    #[inline(always)]
    fn front(&self) -> Option<&T> {
        LinkedList::<T>::front(self)
    }
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        LinkedList::<T>::back(self)
    }
    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        LinkedList::<T>::front_mut(self)
    }
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        LinkedList::<T>::back_mut(self)
    }
}

impl<T> Stack<T> for LinkedList<T> {
    #[inline(always)]
    fn push(&mut self, element: T) {
        LinkedList::<T>::push_back(self, element)
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        LinkedList::<T>::pop_back(self)
    }
    #[inline(always)]
    fn top(&self) -> Option<&T> {
        LinkedList::<T>::back(self)
    }
    #[inline(always)]
    fn top_mut(&mut self) -> Option<&mut T> {
        LinkedList::<T>::back_mut(self)
    }
}

impl<T> Queue<T> for LinkedList<T> {
    #[inline(always)]
    fn enqueue(&mut self, element: T) {
        LinkedList::<T>::push_back(self, element)
    }
    #[inline(always)]
    fn dequeue(&mut self) -> Option<T> {
        LinkedList::<T>::pop_front(self)
    }
    #[inline(always)]
    fn peek(&self) -> Option<&T> {
        LinkedList::<T>::front(self)
    }
    #[inline(always)]
    fn peek_mut(&mut self) -> Option<&mut T> {
        LinkedList::<T>::front_mut(self)
    }
}

impl<T> Insert<usize, T> for LinkedList<T> {
    type Output = ();

    #[inline]
    fn insert(&mut self, index: usize, element: T) {
        if index == 0 {
            LinkedList::<T>::push_front(self, element);
        } else if index < LinkedList::<T>::len(self) {
            let mut front = LinkedList::<T>::split_off(self, index);
            front.push_back(element);
            LinkedList::<T>::append(self, &mut front);
        } else {
            LinkedList::<T>::push_back(self, element);
        }
    }
}

impl<T> Remove<usize> for LinkedList<T> {
    type Output = T;

    #[inline]
    fn remove(&mut self, index: usize) -> Self::Output {
        if index == 0 {
            LinkedList::<T>::pop_front(self).expect("index out of bounds")
        } else if index < LinkedList::<T>::len(self) {
            let mut front = LinkedList::<T>::split_off(self, index);
            let element = front.pop_back();
            LinkedList::<T>::append(self, &mut front);
            element.expect("index out of bounds")
        } else {
            LinkedList::<T>::pop_back(self).expect("index out of bounds")
        }
    }
}

impl<T> Get<usize> for LinkedList<T> {
    type Output = T;

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Output> {
        LinkedList::<T>::iter(self).nth(index)
    }
}
impl<T> GetMut<usize> for LinkedList<T> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        LinkedList::<T>::iter_mut(self).nth(index)
    }
}
