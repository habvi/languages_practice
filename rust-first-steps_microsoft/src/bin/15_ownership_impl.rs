fn run_1() {
    let name = String::from("Alice");
    // transfer ownership name -> name2
    let name2 = name;
    // error : value borrowed here after erase
    // println!("{}", name);
    println!("{}", name2);
}

// ----------------------------------------------------
fn process(input: String) {
    println!("{}", input);
}

fn process2(input: u32) {
    println!("{}", input);
}

fn run_2() {
    {
        let s = String::from("hello");
        // ownership s moved into process
        process(s);
        // error : value used here after erase
        // process(s);
    }
    {
        let n = 1u32;
        // ownership n copied into process2
        process2(n);
        // n can be used again because it was just copied, wasn't moved.
        process2(n);
    }
    {
        // use clone
        let s = String::from("hello world");
        process(s.clone());
        // s was never moved
        process(s);
    }
}

// ----------------------------------------------------
fn print_greeting(message: &String) {
    println!("{}", message);
}

// fn change(message: &String) {
//     message.push_str("!!");
// }

fn change2(message: &mut String) {
    message.push_str("!!");
}

fn run_3() {
    // reference and borrowing
    // only use either &T or &mut T
    let greeting = String::from("hello");
    let greeting_reference = &greeting;
    println!("{} {}", greeting, greeting_reference);

    print_greeting(&greeting);
    print_greeting(greeting_reference);

    // error : not mutable
    // change(&greeting);

    let mut greeting = String::from("hello");
    change2(&mut greeting);
    println!("{}", greeting);
}

// ----------------------------------------------------
fn run_4() {
    #[allow(unused_variables)]
    let x;
    #[allow(unused_assignments)]
    {
        let y = 20;
        x = &y;
    }
    // error : y already dropped
    // println!("x : {}", x);
}

// ----------------------------------------------------
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

fn run_5() {
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
}

// ----------------------------------------------------
#[derive(Debug)]
struct Highlight<'document>(&'document str);

#[allow(dead_code)]
fn erase(s: String) {
    println!("{}", s);
}

fn run_6() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?} {:?}", fox, dog);

    // if transfer the text to function, error
    // erase(text);
    // println!("{:?} {:?}", fox, dog);
}

// ----------------------------------------------------
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn init(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

fn run_7() {
    let mut p: Person = Person::init("John", "Doe");
    println!("{:?} {} {}", p, p.first_name, p.last_name);
    println!("{}", p.full_name());

    p = Person::init("Alice", "K");
    println!("{}", p.full_name());
    p.set_last_name("M");
    println!("{} {:?}", p.full_name(), p.to_tuple());
}

// ----------------------------------------------------
fn main() {
    run_1();
    println!("-----");

    run_2();
    println!("-----");

    run_3();
    println!("-----");

    run_4();
    println!("-----");

    run_5();
    println!("-----");

    run_6();
    println!("-----");

    run_7();
    println!("-----");
}
