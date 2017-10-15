use super::Collection;


pub trait CollectionMut: Collection {
    fn clear(&mut self);
}
