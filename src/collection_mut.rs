

pub trait CollectionMut {
    fn len(&self) -> usize;
    fn clear(&mut self);
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
