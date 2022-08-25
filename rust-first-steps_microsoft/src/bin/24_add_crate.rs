// add: regex -> Cargo.toml
// $ cargo build

use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("date match? : {}", re.is_match("2022-05-14"));
}
