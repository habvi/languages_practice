fn main() {
    let a = true;
    let b = if a { "Yes" } else { "No" };
    assert_eq!(b, "Yes");
}
