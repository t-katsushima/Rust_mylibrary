// TODO: 演算子の引数が負の場合に対応

use std::ops::*;

#[derive(Debug)]
struct ModIsize {
    n: isize,
    p: isize,
}

impl ModIsize {
    fn new(n: isize, p: isize) -> Self {
        if n < 0 {
            Self { n: p + (n % p), p }
        } else {
            Self { n: n % p, p }
        }
    }

    fn to_isize(self) -> isize {
        self.n
    }

    fn plus(&self, that: isize) -> Self {
        Self::new((self.n + (that % self.p)) % self.p, self.p)
    }
    fn minus(&self, that: isize) -> Self {
        Self::new(((self.n + self.p) - (that % self.p)) % self.p, self.p)
    }
    fn multi(&self, that: isize) -> Self {
        Self::new((self.n * (that % self.p)) % self.p, self.p)
    }
}

impl Add<isize> for ModIsize {
    type Output = ModIsize;
    fn add(self, rhs: isize) -> Self::Output {
        self.plus(rhs)
    }
}
impl Sub<isize> for ModIsize {
    type Output = ModIsize;
    fn sub(self, rhs: isize) -> Self::Output {
        self.minus(rhs)
    }
}
impl Mul<isize> for ModIsize {
    type Output = ModIsize;
    fn mul(self, rhs: isize) -> Self::Output {
        self.multi(rhs)
    }
}

impl AddAssign<isize> for ModIsize {
    fn add_assign(&mut self, rhs: isize) {
        self.n += rhs % self.p;
        self.n %= self.p;
    }
}
