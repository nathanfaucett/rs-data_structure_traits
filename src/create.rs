

pub trait Create {
    fn create() -> Self;
    fn create_with_capacity(capacity: usize) -> Self;
}
