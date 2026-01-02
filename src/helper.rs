pub fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = x.div_ceil(2) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
