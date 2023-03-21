pub trait ExtVec<T: Clone> {
    fn head_option(&self) -> Option<T>;
}
impl<T: Clone> ExtVec<T> for Vec<T> {
    fn head_option(&self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            Some(self[0].clone())
        }
    }
}
pub trait ExtVecDisplay<T: core::fmt::Display + ?Sized> {
    fn joining(&self, sep: &str) -> String;
}
impl<T: core::fmt::Display> ExtVecDisplay<T> for Vec<T> {
    fn joining(&self, sep: &str) -> String {
        self.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}
