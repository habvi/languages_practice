fn main() {
    let v = vec![10, 20, 30];

    let mut itr = v.iter().enumerate();
    assert_eq!(itr.next(), Some((0, &10)));
    assert_eq!(itr.next(), Some((1, &20)));
    assert_eq!(itr.next(), Some((2, &30)));
    assert_eq!(itr.next(), None);

    let a: Vec<_> = v.iter().enumerate().collect();
    assert_eq!(a, [(0, &10), (1, &20), (2, &30)]);

    // same
    {
        for (i, x) in v.iter().enumerate() {
            println!("{} {}", i, x);
        }

        for (i, x) in (0..3).zip(v.iter()) {
            println!("{} {}", i, x);
        }
    }

    let v: Vec<_> = (0..3).map(|i| (i, 0)).collect();
    assert_eq!(v, [(0, 0), (1, 0), (2, 0)]);
}
