// 返り値: a と b の最大公約数
// ax + by = gcd(a, b) を満たす (x, y) が格納される
fn ext_gcd(a: isize, b: isize, x: &mut isize, y: &mut isize) -> isize {
    if b == 0 {
        *x = 1;
        *y = 0;
        a
    } else {
        let d = ext_gcd(b, a % b, y, x);
        *y -= a / b * *x;
        d
    }
}
