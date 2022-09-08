fn main() {
    let v: Vec<usize> = vec![5, 1, 3, 4, 2];
    // sort
    {
        let mut a: Vec<usize> = v.iter().cloned().collect();
        a.sort();
        assert_eq!(a, [1, 2, 3, 4, 5]);
        a.sort_by(|x, y| x.cmp(y));
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }
    // sort reverse
    {
        use std::cmp::Reverse;
        let mut a: Vec<usize> = v.iter().cloned().collect();
        a.sort_by_key(|&x| Reverse(x));
        assert_eq!(a, [5, 4, 3, 2, 1]);
        a.sort_by(|x, y| y.cmp(x));
        assert_eq!(a, [5, 4, 3, 2, 1]);
    }
    // reverse
    {
        let mut a: Vec<usize> = v.iter().cloned().collect();
        a.reverse();
        assert_eq!(a, [2, 4, 3, 1, 5]);
    }
    // rev
    {
        let a: Vec<usize> = v.iter().rev().cloned().collect();
        assert_eq!(a, [2, 4, 3, 1, 5]);
    }

    // sort slice
    {
        use std::cmp::Reverse;
        let mut v = vec![(40, 1), (10, 2), (50, 3), (60, 4), (20, 5), (30, 6)];
        v[..3].sort_by_key(|&(x, _)| Reverse(x));
        v[3..].sort();
        assert_eq!(v, [(50, 3), (40, 1), (10, 2), (20, 5), (30, 6), (60, 4)]);
    }
    // sort_by, sort_by_key
    {
        let mut v = vec![(20, 1), (10, 2), (30, 3), (40, 4)];
        v.sort_by_key(|x| x.0);
        assert_eq!(v, [(10, 2), (20, 1), (30, 3), (40, 4)]);
        v.sort_by(|a, b| a.cmp(b));
        assert_eq!(v, [(10, 2), (20, 1), (30, 3), (40, 4)]);

        v.sort_by_key(|x| x.1);
        assert_eq!(v, [(20, 1), (10, 2), (30, 3), (40, 4)]);
        v.sort_by(|a, b| a.1.cmp(&(b.1)));
        assert_eq!(v, [(20, 1), (10, 2), (30, 3), (40, 4)]);

        v.sort_by_key(|x| -x.1);
        assert_eq!(v, [(40, 4), (30, 3), (10, 2), (20, 1)]);
        v.sort_by(|a, b| (-a.1).cmp(&(-b.1)));
        assert_eq!(v, [(40, 4), (30, 3), (10, 2), (20, 1)]);
    }
    // 2 keys
    {
        use std::cmp::Reverse;
        let mut v = vec![(20, 3), (10, 3), (30, 1), (10, 2), (30, 3)];
        v.sort_by_key(|&(a, b)| (Reverse(b), a));
        assert_eq!(v, [(10, 3), (20, 3), (30, 3), (10, 2), (30, 1)]);
    }
}
