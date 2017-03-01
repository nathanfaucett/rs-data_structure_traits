

pub trait Collection {
    fn len(&self) -> usize;
    fn clear(&mut self);
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
