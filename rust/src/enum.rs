#[derive(Debug)]
enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn run1() {
    let v4 = IpAddKind::V4(127, 0, 0, 1);
    let v6 = IpAddKind::V6(String::from("::1"));
    println!("{:?}", v4);
    println!("{:?}", v6);
}

// --------------------------------
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Message {
        self
    }
}

fn run2() {
    let m = Message::Write(String::from("abcd"));
    println!("{:?} {:?}", m, m.call());
    // how to get "abcd" ..?

}

// --------------------------------
fn main() {
    run1();
    println!("-----");
    run2();
}
