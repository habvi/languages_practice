#[allow(unused_variables)]
fn main() {
    // stack memory
    let x = 5;
    // auto copy
    let y = x;
    let s = "abc";
    /*
    Copy trait:
    integer, bool, float, char, tuple(just Copy trait types)
    */

    // allocate heap memory
    let s1 = String::from("abc");
    // move s1's ownership
    let s2 = s1;
    // copy
    let s3 = s2.clone();
    assert_eq!("abc", s2);
    assert_eq!("abc", s3);

    // & : reference, one &mut or multiple &
    let mut s = String::from("abc");
    // borrow
    let a = &mut s;
    a.push_str("def");
    // or
    // let b = &s;
    // let c = &s;
}
