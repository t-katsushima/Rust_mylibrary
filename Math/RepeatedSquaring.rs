// mod p で x^n
fn repeated_squaring(x: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        repeated_squaring((x * x) % p, n / 2, p)
    } else {
        (x * repeated_squaring(x, n - 1, p)) % p
    }
}
