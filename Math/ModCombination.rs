struct ModComb {
    fact_array: Vec<usize>,
    fact_inv_array: Vec<usize>,
    modulo: usize,
}

impl ModComb {
    fn new(max_num: usize, modulo: usize) -> ModComb {
        let mut fact = vec![0; max_num + 1];
        fact[0] = 1;
        for i in 1..=max_num {
            fact[i] = (i * fact[i - 1]) % modulo;
        }

        fn repeated_squaring(x: usize, n: usize, p: usize) -> usize {
            if n == 0 {
                1
            } else if n % 2 == 0 {
                repeated_squaring(x * x % p, n / 2, p)
            } else {
                (x * repeated_squaring(x, n - 1, p)) % p
            }
        }

        let mut fact_inv = vec![0; max_num + 1];
        fact_inv[max_num] = repeated_squaring(fact[max_num], modulo - 2, modulo);
        for i in (1..=max_num).rev() {
            fact_inv[i - 1] = (i * fact_inv[i]) % modulo;
        }

        ModComb {
            fact_array: fact,
            fact_inv_array: fact_inv,
            modulo,
        }
    }

    // nCk
    fn comb(&self, n: usize, k: usize) -> usize {
        let numer = self.fact_array[n];
        let denom = (self.fact_inv_array[n - k] * self.fact_inv_array[k]) % self.modulo;

        (numer * denom) % self.modulo
    }
}