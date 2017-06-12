use super::collection::Collection;


pub trait CollectionMut: Collection {
    fn clear(&mut self);
}
