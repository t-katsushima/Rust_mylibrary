// 未テスト

use std::ops::*;

struct ModUsize {
    n: usize,
    p: usize,
}

impl ModUsize {
    fn new(n: usize, p: usize) -> Self {
        Self { n: n % p, p }
    }

    fn to_usize(self) -> usize {
        self.n
    }

    fn plus(&self, that: usize) -> Self {
        Self::new((self.n + (that % self.p)) % self.p, self.p)
    }
    fn minus(&self, that: usize) -> Self {
        Self::new(((self.n + self.p) - (that % self.p)) % self.p, self.p)
    }
    fn multi(&self, that: usize) -> Self {
        Self::new((self.n * (that % self.p)) % self.p, self.p)
    }
}

impl Add<usize> for ModUsize {
    type Output = ModUsize;
    fn add(self, rhs: usize) -> Self::Output {
        self.plus(rhs)
    }
}
impl Sub<usize> for ModUsize {
    type Output = ModUsize;
    fn sub(self, rhs: usize) -> Self::Output {
        self.minus(rhs)
    }
}
impl Mul<usize> for ModUsize {
    type Output = ModUsize;
    fn mul(self, rhs: usize) -> Self::Output {
        self.multi(rhs)
    }
}

impl AddAssign<usize> for ModUsize {
    fn add_assign(&mut self, rhs: usize) {
        self.n += rhs % self.p;
        self.n %= self.p;
    }
}