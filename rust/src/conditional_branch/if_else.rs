fn main() {
    {
        let a = true;
        let b = if a { "Yes" } else { "No" };
        assert_eq!(b, "Yes");
    }
    {
        let v = [10, 4, 2];
        println!("{}", if v.iter().all(|&x| x >= 1) { "Yes" } else { "No" });

        println!(
            "{}",
            if v.iter().all(|x| *x % 2 == 0) {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
