fn main() {
    let mut v: Vec<usize> = vec![5, 1, 3, 4];
    v.reverse();
    assert_eq!(vec![4, 3, 1, 5], v);

    let v2: Vec<usize> = v.iter().rev().cloned().collect();
    assert_eq!(vec![5, 1, 3, 4], v2);

    v.sort();
    assert_eq!(vec![1, 3, 4, 5], v);

    use std::cmp::Reverse;
    v.sort_by_key(|&x| Reverse(x));
    assert_eq!(vec![5, 4, 3, 1], v);
}
