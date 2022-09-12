// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let some_num = Some(8);
    assert_eq!(some_num.is_some(), true);

    let some_str = Some("abc");
    assert_eq!(some_str.map(|s| s.len()), Some(3));

    // None needs type
    let none: Option<&str> = None;
    assert_eq!(none.is_none(), true);

    // unwrap
    assert_eq!(some_num.unwrap(), 8);
    assert_eq!(none.unwrap_or("empty.."), "empty..");
}
