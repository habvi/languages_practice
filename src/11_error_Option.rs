fn main() {
    // panic!("farewell!");

    let v = vec![0, 1, 2];
    // index out of bounds
    // println!("{}", v[3]);

    // return Option
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    println!("{:?} {:?}", v.get(2), v.get(5));
    println!("-----");

    // match : must cover all patterns
    let fruits = vec!["Apple", "Lemon", "Orange"];
    for &i in [0, 2, 50].iter() {
        match fruits.get(i) {           // -> Option<&str>
            Some(&"Orange") => println!("Orange is awesome!"),
            Some(fruit_name) => println!("{}", fruit_name),
            None => println!("There is no fruit!"),
        }
    }
    println!("-----");

    // _ : wild card
    let num: Option<u8> = Some(7);
    match num {
        Some(7) => println!("Lucky number"),
        _ => {},
    }

    // if let
    let num: Option<u8> = Some(7);
    if let Some(7) = num {
        println!("Lucky number");
    }
    println!("-----");


    // unwrap
    let gift = Some("candy");
    // access to Option value
    assert_eq!(gift.unwrap(), "candy");

    #[allow(unused_variables)]
    let empty_gift: Option<&str> = None;
    // None can't unwrap -> panic
    // assert_eq!(empty_gift.unwrap(), "candy");


    // expect
    let a = Some("value");
    assert_eq!(a.expect("custom panic massage!"), "value");

    #[allow(unused_variables)]
    let b: Option<&str> = None;
    // panic
    // b.expect("custom panic massage!");

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}