use itertools::izip;

fn main() {
    // izip
    {
        let mut v = vec![5, 1, 7, 8];
        let mut a: Vec<_> = izip!(v, 1..).collect();
        // let mut a = izip!(v, 1..).collect::<Vec<_>>();
        assert_eq!(a, [(5, 1), (1, 2), (7, 3), (8, 4)]);
    }
}