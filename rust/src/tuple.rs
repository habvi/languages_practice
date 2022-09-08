fn main() {
    let tup: (i32, &str, f64, u8) = (10, "abc", 5.8, 1);

    #[allow(unused_variables)]
    let (a, b, c, d) = tup;

    assert_eq!(tup.0, 10,);
    assert_eq!(tup.1, "abc");
    assert_eq!(tup.2, 5.8);
    assert_eq!(tup.3, 1,);
}
