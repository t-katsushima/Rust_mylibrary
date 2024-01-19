// MaxN : 2の乗数でないといけない。要素数に応じた値。(e.g. 10^5なら1<<17にしておくとよい)
struct RMQ {
    n: usize,
    dat: Vec<isize>,
}

impl RMQ {
    // n: 要素数
    fn new(n: usize) -> Self {
        // 簡単のため、要素数を2の冪乗に
        let mut nn = 1;
        while nn < n {
            nn *= 2;
        }

        RMQ {
            n: nn,
            dat: vec![std::isize::MAX; nn * 2 - 1],
        }
    }

    // idx番目の値(0-indexed)をaに変更
    fn update(&mut self, idx: usize, a: isize) {
        // 葉の節点
        let mut k = idx + (self.n - 1);
        self.dat[k] = min(self.dat[k], a);

        // 登りながら更新
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = min(self.dat[k * 2 + 1], self.dat[k * 2 + 2]);
        }
    }

    // [a, b) の最小値を求める
    // 後ろの方の引数は、計算の簡単のための引数
    // kは節点の番号。l, r はその節点が[l, r) に対応づいていることを表す。
    fn query(&self, a: usize, b: usize) -> isize {
        fn func(s: &RMQ, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
            if r <= a || b <= l {
                // [a, b)と[l, r)が交差しなければ std::isize::MAX
                std::isize::MAX
            }
            // [a, b)が[l, r)を完全に含んでいれば、この節点の値
            else if a <= l && r <= b {
                s.dat[k]
            }
            // そうでなければ、２つの子の最小値
            else {
                let vl = func(&s, a, b, 2 * k + 1, l, (l + r) / 2);
                let vr = func(&s, a, b, 2 * k + 2, (l + r) / 2, r);

                min(vl, vr)
            }
        }

        func(&self, a, b, 0, 0, self.n)
    }
}
