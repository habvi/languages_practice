fn comb(n: usize, r: usize) -> usize {
    let n1: usize = n + 1;
    let r: usize = min(r, n - r);
    let mut nmrt: usize = 1;
    let mut dnmnt: usize = 1;
    for i in 1..=r {
        nmrt = nmrt * (n1 - i);
        dnmnt *= i;
    }
    nmrt / dnmnt
}
