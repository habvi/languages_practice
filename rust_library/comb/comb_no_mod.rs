use std::cmp::min;

fn comb(n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    let r: usize = min(r, n - r);
    let mut nmrt: usize = 1;
    let mut dnmnt: usize = 1;
    for i in 1..=r {
        nmrt = nmrt * (n + 1 - i);
        dnmnt *= i;
    }
    nmrt / dnmnt
}
