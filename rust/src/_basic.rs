#[allow(unused_variables, unused_mut)]
fn main() {
    print!("");
    println!("");
    eprint!("");
    eprintln!("");

    let a = 0;
    let mut b = 0;

    let a = {
        let b = 5;
        b * 2
    };
    assert_eq!(a, 10);
}
