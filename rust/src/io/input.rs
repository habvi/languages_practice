use std::io::stdin;
use std::str::FromStr;

fn run1() {
    let mut s: String = String::new();
    let result: std::io::Result<usize> = stdin().read_line(&mut s);
    match result {
        Ok(_) => println!("> {}", s),
        Err(err) => println!("> {}", err),
    }
}
/*
abc
 */

// --------------------------------------------------
// "abc" -> "abc"
fn input_s() -> String {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().to_string()
}

// "123" -> 123
// -> u32, usize..
fn input_i() -> i64 {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

// "abc" -> "abc", "123" -> 123
fn input_t<T: FromStr>() -> T {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn run2() {
    let s1: String = input_s();
    let s2: String = input_t();
    let x1: i64 = input_i();
    let x2: usize = input_t();
    println!("> {} {} {} {}", s1, s2, x1, x2);
}
/*
abc
abc
123
123
 */

// --------------------------------------------------
// "abc" -> ['a', 'b', 'c']
fn input_vec_c() -> Vec<char> {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().chars().collect()
}

// "1 2 3" -> [1, 2, 3]
// -> Vec<usize>..
fn input_vec_i() -> Vec<i64> {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim()
    .split_whitespace()
    .map(|e| e.parse().ok().unwrap())
    .collect()
}

// "a bc def" -> ["a", "bc", "def"], "1 2 3" -> [1, 2, 3]
fn input_vec<T: FromStr>() -> Vec<T> {
    input_t::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

// "ab cde f" -> [['a', 'b'], ['c', 'd', 'e'], ['f']]
fn input_vec_vc() -> Vec<Vec<char>> {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.split_whitespace().map(|e| e.chars().collect()).collect()
}

fn run3() {
    let v: Vec<char> = input_vec_c();
    println!("> {:?}", v);
    let v: Vec<String> = input_vec();
    println!("> {:?}", v);
    let v: Vec<i64> = input_vec_i();
    println!("> {:?}", v);
    let v: Vec<usize> = input_vec();
    println!("> {:?}", v);

    let vc: Vec<Vec<char>> = input_vec_vc();
    println!("> {:?}", vc);
}
/*
abcd
a bc def
1 23 456
1 23 456
ab cde f
 */

// --------------------------------------------------
// "1 2 3" -> (1, 2, 3)
// chage lines
fn input_tuple<T: FromStr>() -> (T, T, T) {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    let mut itr = s.split_whitespace();
    (
        itr.next().unwrap().parse().ok().unwrap(),
        itr.next().unwrap().parse().ok().unwrap(),
        itr.next().unwrap().parse().ok().unwrap()
    )
}

fn run4() {
    let (a, b, c): (usize, usize, usize) = input_tuple();
    println!("> {} {} {}", a, b, c);
    let (a, b, c): (String, String, String) = input_tuple();
    println!("> {} {} {}", a, b, c);

    let nm: Vec<usize> = input_vec();
    let (n, m): (usize, usize) = (nm[0], nm[1]);
    println!("> {} {}", n, m);

    let (n, m) = {
        let nm: Vec<usize> = input_vec();
        (nm[0], nm[1])
    };
    println!("> {} {}", n, m);
}
/*
1 23 456
ab12 100 55555
7 8
9 10
*/

// --------------------------------------------------
// -> [1, 5, 22],  -> ["abcd", "ef", "g"]
fn input_ts<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| {
        let mut s: String = String::new();
        stdin().read_line(&mut s).ok().unwrap();
        s.trim().parse().ok().unwrap()
    })
    .collect()
}

fn run5() {
    let n: usize = input_t();

    let v: Vec<usize> = input_ts(n);
    println!("{:?}", v);
    let v: Vec<String> = input_ts(n);
    println!("{:?}", v);
}
/*
3
1
5
22
abcd
ef
g
 */

 // --------------------------------------------------
// -> [(1, 5, 22), (9, 10, 3)]
fn input_nlines_tuple(n: usize) -> Vec<(usize, usize, usize)> {
    (0..n).map(|_| input_tuple()).collect()
}

// -> [[1, 5, 22], [9, 10, 3]]
fn input_nlines_vec<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| input_vec()).collect()
}

fn run6() {
    let n: usize = input_t();

    let vt: Vec<(usize, usize, usize)> = input_nlines_tuple(n);
    println!("{:?} {}", vt, vt[1].2);
    let vv: Vec<Vec<usize>> = input_nlines_vec(n);
    println!("{:?}", vv);
    let vv: Vec<Vec<String>> = input_nlines_vec(n);
    println!("{:?} {}", vv, vv[1][0]);
}
/*
2
1 5 22
9 10 3
1 5 22
9 10 3
a bc def
gh i jklm
*/

 // --------------------------------------------------
fn main() {
    // standard
    // run1();

    // functionalize
    // String, integer
    // run2();

    // Vec
    // run3();

    // tuple
    // run4();

    // n lines vec
    // run5();

    // n lines 2D vec
    run6();
}
