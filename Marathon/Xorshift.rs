// cf. https://qiita.com/hatoo@github/items/652b81e8e83b0680bc0a#xorshift

#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}
#[allow(dead_code)]
impl Xorshift {
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }

    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed }
    }

    #[inline]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }

    // [0, m) からランダムに返す
    #[inline]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }

    #[inline]
    // 0.0 ~ 1.0
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}
