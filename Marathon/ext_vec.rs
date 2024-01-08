mod ext_vec {
    pub trait ExtVec<T: Clone> {
        // Vec::first で良さそう
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

    pub trait ExtVecOrd<T: Ord> {
        fn sort_desc(&mut self);
    }
    impl<T: Ord> ExtVecOrd<T> for Vec<T> {
        fn sort_desc(&mut self) {
            self.sort_by(|a, b| b.cmp(a));
        }
    }

    pub trait ExtVecDisplay<T: core::fmt::Display> {
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
}

use ext_vec::*;
