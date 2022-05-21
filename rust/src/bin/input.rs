use std::io::stdin;

#[allow(dead_code)]
fn input_s() -> String {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().to_string()
}

#[allow(dead_code)]
fn input_i() -> usize {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn input_vec_i() -> Vec<i64> {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn input_t<T: std::str::FromStr>() -> T {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn input_vec<T: std::str::FromStr>() -> Vec<T> {
    input_t::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    // standard
    {
        let mut s: String = String::new();
        let result: std::io::Result<usize> = stdin().read_line(&mut s);
        match result {
            Ok(_) => println!("{}", s),
            Err(err) => println!("{}", err),
        }
    }

    // functionalize
    {
        let s: String = input_t();
        let x: usize = input_t();
        println!("{} {}", s, x);

        let v: Vec<String> = input_vec();
        println!("{:?}", v);

        let vi: Vec<usize> = input_vec();
        println!("{:?}", vi);
    }
}
