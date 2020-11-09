// 未テスト

// 条件を満たす最大の値を返す
fn binary_search(_min: isize, _max: isize) -> isize {
    fn check(n: isize) -> bool {
        unimplemented!()
    }

    fn func(min: isize, max: isize) -> isize {
        let mid: isize = (max + min) / 2;
        match max - min {
            0 | 1 => match check(max) {
                true => max,
                false => min,
            },
            _ => match check(mid) {
                true => func(mid, max),
                false => func(min, mid),
            },
        }
    }

    func(_min, _max)
}

// 条件を満たす最小の値を返す
fn binary_search(_min: isize, _max: isize) -> isize {
    fn check(n: isize) -> bool {
        unimplemented!()
    }

    fn func(min: isize, max: isize) -> isize {
        let mid: isize = (max + min) / 2;
        match max - min {
            0 | 1 => match check(min) {
                true => min,
                false => max,
            },
            _ => match check(mid) {
                true => func(min, mid),
                false => func(mid, max),
            },
        }
    }

    func(_min, _max)
}
