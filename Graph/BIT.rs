struct BinaryIndexedTree {
    n: usize,
    ar: Vec<isize>,
}

impl BinaryIndexedTree {
    // n is num of element
    fn new(n: usize) -> BinaryIndexedTree {
        BinaryIndexedTree {
            n,
            ar: vec![0; n + 1],
        }
    }

    // sum of [1, i]
    fn sum(&self, i: usize) -> isize {
        fn func(ar: &Vec<isize>, i: usize, res: isize) -> isize {
            if i <= 0 {
                res
            } else {
                func(ar, i - (i as isize & -(i as isize)) as usize, res + ar[i])
            }
        }
        func(&self.ar, i, 0)
    }

    // add x to i-th element
    fn add(&mut self, i: usize, x: isize) {
        fn func(bit: &mut BinaryIndexedTree, i: usize, x: isize) {
            if i <= bit.n {
                bit.ar[i] += x;
                func(bit, i + (i as isize & -(i as isize)) as usize, x)
            }
        }
        func(self, i, x)
    }
}
