// ['a', 'b', 'c', 'd'] -> "abcd"
fn run1() {
    let vc: [char; 4] = ['a', 'b', 'c', 'd'];

    let s: String = vc.iter().collect();
    let s2 = vc.iter().collect::<String>();
    assert_eq!(s, s2);
}

// [1, 4, 7, 10] -> 1 4 7 10,  ["ab", "cde", "fghi"] -> ab cde fghi
fn run2() {
    let v: [usize; 4] = [1, 4, 7, 10];
    println!("{}", v.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));

    let v: [&str; 3] = ["ab", "cde", "fghi"];
    println!("{}", v.join(" "));
    println!("{}", v.join("\n"));
}

// -----------------------------
fn main() {
    run1();
    run2();
}
