fn process(input: String) {
    println!("{}", input);
}

fn process2(input: u32) {
    println!("{}", input);
}

fn print_greeting(message: &String) {
    println!("{}", message);
}

// fn change(message: &String) {
//     message.push_str("!!");
// }

fn change2(message: &mut String) {
    message.push_str("!!");
}

// error : expected lifetime parameter
// fn longest_word(x: &String, y: &String) -> &String {
//     if x.len() > y.len() [
//         x
//     ] else {
//         y
//     }
// }
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Highlight<'document>(&'document str);

#[allow(dead_code)]
fn erase(s: String) {
    println!("{}", s);
}

fn main() {
    let name = String::from("Alice");
    // transfer ownership name -> name2
    let name2 = name;
    // error : value borrowed here after erase
    // println!("{}", name);
    println!("{}", name2);

    let s = String::from("hello");
    // ownership s moved into process
    process(s);
    // error : value used here after erase
    // process(s);

    let n = 1u32;
    // ownership n copied into process2
    process2(n);
    // n can be used again because it was just copied, wasn't moved.
    process2(n);

    // use clone
    let s = String::from("hello world");
    process(s.clone());
    // s was never moved
    process(s);
    println!("-----");

    // reference and borrowing
    // only use either &T or &mut T
    let greeting = String::from("hello");
    let greeting_reference = &greeting;
    println!("{} {}", greeting, greeting_reference);

    print_greeting(&greeting);
    print_greeting(greeting_reference);
    println!("-----");

    // error : not mutable
    // change(&greeting);

    let mut greeting = String::from("hello");
    change2(&mut greeting);
    println!("{}", greeting);

    #[allow(unused_variables)]
    let x;
    #[allow(unused_assignments)]
    {
        let y = 20;
        x = &y;
    }
    // error : y already dropped
    // println!("x : {}", x);

    let magic1 = String::from("abracadabra");
    let magic2 = String::from("shazam");
    let result = longest_word(&magic1, &magic2);
    println!("{}", result);

    // error : magic2 doesn't live long enough
    // let result;
    // {
    //     let magic2 = String::from("shazam");
    //     result = longest_word(&magic1, &magic2);
    // }
    // println!("{}", result);
    println!("-----");

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?} {:?}", fox, dog);

    // if transfer the text to function, error
    // erase(text);
    // println!("{:?} {:?}", fox, dog);
}
