pub trait ExtBTreeSet<T: Clone> {
    fn to_vec(&self) -> Vec<T>;
}
impl<T: Clone> ExtBTreeSet<T> for BTreeSet<T> {
    fn to_vec(&self) -> Vec<T> {
        self.clone().into_iter().collect::<Vec<_>>()
    }
}
