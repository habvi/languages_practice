fn main() {
    use std::mem::swap;

    let s: Vec<char> = "abcdef".chars().collect();

    let mut a: Vec<char> = s[..3].to_vec();
    let mut b: Vec<char> = s[3..].to_vec();
    assert_eq!(a, ['a', 'b', 'c']);
    assert_eq!(b, ['d', 'e', 'f']);

    a.swap(0, 1);
    assert_eq!(a, ['b', 'a', 'c']);

    swap(&mut a[2], &mut b[2]);
    assert_eq!(a, ['b', 'a', 'f']);
    assert_eq!(b, ['d', 'e', 'c']);

    swap(&mut a, &mut b);
    assert_eq!(a, ['d', 'e', 'c']);
    assert_eq!(b, ['b', 'a', 'f']);
}