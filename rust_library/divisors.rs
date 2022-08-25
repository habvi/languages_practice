fn div_list(x: usize) -> Vec<usize> {
    let mut div_l: Vec<usize> = Vec::new();
    let mut div_r: Vec<usize> = Vec::new();
    let mut i: usize = 1;
    while i * i <= x {
        if x % i == 0 {
            div_l.push(i);
            if i != x / i {
                div_r.push(x / i);
            }
        }
        i += 1;
    }
    // div_r.reverse();
    // div_l.append(&mut div_r);
    // div_l
    div_l.iter().cloned().chain(div_r.iter().rev().cloned()).collect()
}
