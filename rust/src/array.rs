fn main() {
    let a: [&str; 3] = ["ab", "cde", "fghi"];
    let mut x = a.len();
    assert_eq!(x, 3);

    let mut a = [0; 5];
    a[2] = 10;
    x = std::mem::size_of_val(&a);
    assert_eq!(x, 20); // 4bytes * 5 = 20

    // slice
    {
        let b: &[i32] = &a;
        assert_eq!(b, [0, 0, 10, 0, 0]);
        let b: &[i32] = &a[0..3];
        assert_eq!(b, [0, 0, 10]);
    }
}
