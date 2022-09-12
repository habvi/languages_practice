use itertools::izip;

fn main() {
    // izip
    {
        let v = vec![5, 3, 7, 8];
        let mut a: Vec<_> = izip!(v.clone(), 0..).collect();
        assert_eq!(a, [(5, 0), (3, 1), (7, 2), (8, 3)]);

        a = izip!(v, 1..).collect::<Vec<_>>();
        assert_eq!(a, [(5, 1), (3, 2), (7, 3), (8, 4)]);
    }
}