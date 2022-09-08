fn main() {
    // 0..n
    {
        let v: Vec<usize> = (0..5).collect();
        assert_eq!(v, [0, 1, 2, 3, 4]);
        let v = (0..5).collect::<Vec<_>>();
        assert_eq!(v, [0, 1, 2, 3, 4]);

        for i in 0..5 {
            print!("{} ", i);
        }
        println!("");

        for i in (0..5).rev() {
            print!("{} ", i);
        }
        println!("");
    }
}