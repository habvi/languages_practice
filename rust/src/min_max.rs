fn main() {
    {
        use std::cmp::min;
        use std::cmp::max;

        let mn = min(5, 10);
        assert_eq!(mn, 5);
        let mx = max(944, 12747);
        assert_eq!(mx, 12747);
    }
    {
        let v: Vec<usize> = vec![1, 2, 3, 4, 5];

        // -> Option
        let mn = v.iter().min().unwrap();
        assert_eq!(mn, &1);
        let mx = v.iter().max().unwrap();
        assert_eq!(mx, &5);
    }
}
