// 未テスト

// 条件を満たす最大の値を返す
struct BinarySearch {}

impl BinarySearch {
    fn check(&self, n: isize) -> bool {
        unimplemented!()
    }

    fn solve(&self, min: isize, max: isize) -> isize {
        let mid: isize = (max + min) / 2;
        match max - min {
            0 | 1 => match self.check(max) {
                true => max,
                false => min,
            },
            _ => match self.check(mid) {
                true => self.solve(mid, max),
                false => self.solve(min, mid),
            },
        }
    }
}

// 条件を満たす最小の値を返す
struct BinarySearch {}

impl BinarySearch {
    fn check(&self, n: isize) -> bool {
        unimplemented!()
    }

    fn solve(&self, min: isize, max: isize) -> isize {
        let mid: isize = (max + min) / 2;
        match max - min {
            0 | 1 => match self.check(min) {
                true => min,
                false => max,
            },
            _ => match self.check(mid) {
                true => self.solve(min, mid),
                false => self.solve(mid, max),
            },
        }
    }
}
