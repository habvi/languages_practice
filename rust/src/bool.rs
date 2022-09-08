fn main() {
    let is_bigger: bool = 1 > 4;
    assert_eq!(is_bigger, false);

    let a = true;
    assert_eq!(a as u8, 1);
    assert_eq!(!a, false);
    assert_eq!(!a as u8, 0);
}