fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_func_test {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 1), 2);
        // failed
        // assert_eq!(add(1, 1), 500);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(1, 1), 500);
        // failed
        // assert_eq!(add(1, 1), 2);
    }

    #[test]
    #[ignore = "not check yet"]
    fn add_negatives() {
        assert_eq!(add(-5, -5), -10);
    }
}
