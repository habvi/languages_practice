struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("hello").value, "hello");
    assert_eq!(Container::new(String::from("hi")).value, String::from("hi"));
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}