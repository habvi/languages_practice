use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let word: String = args[1].clone();
    println!("first args: {}", word);
}
