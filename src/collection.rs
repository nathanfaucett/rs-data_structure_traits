

pub trait Collection {
    fn len(&self) -> usize;
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
