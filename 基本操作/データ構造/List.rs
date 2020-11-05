enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

#[allow(dead_code)]
impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn cons(self, e: T) -> List<T> {
        List::Cons(e, Box::new(self))
    }

    fn to_vec(self) -> Vec<T> {
        fn func<T>(lst: List<T>, mut res: Vec<T>) -> Vec<T> {
            match lst {
                List::Nil => res,
                List::Cons(e, rest) => {
                    res.push(e);
                    func(*rest, res)
                }
            }
        }

        func(self, Vec::new())
    }
}
