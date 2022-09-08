fn main() {
    // String -> integer
    {
        let s: String = String::from("12345");

        let x: usize = s.parse().unwrap();
        assert_eq!(x, 12345);
        let x = s.parse::<usize>().unwrap();
        assert_eq!(x, 12345);
    }
}
