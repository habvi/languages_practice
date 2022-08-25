#[allow(unused_variables, unused_mut)]
fn main() {
    print!("");
    println!("");
    eprint!("");
    eprintln!("");

    let a = 0;
    let mut b = 0;

    // bool
    {
        let is_bigger = 1 > 4;
        let a = true;
        assert_eq!(!a, false);
        assert_eq!(!a as u8, 0);
        assert_eq!(a as u8, 1);
    }
}
