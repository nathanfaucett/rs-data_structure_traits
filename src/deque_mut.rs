use super::collection_mut::CollectionMut;


pub trait DequeMut<T>: CollectionMut {
    fn push_front(&mut self, element: T);
    fn push_back(&mut self, element: T);

    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;

    fn front(&self) -> Option<&T>;
    fn back(&self) -> Option<&T>;

    fn front_mut(&mut self) -> Option<&mut T>;
    fn back_mut(&mut self) -> Option<&mut T>;
}
