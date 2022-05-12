fn goodbye(message: &str) {
    println!("{}", message);
}

// no need ";"
fn devide_by_5(num: u32) -> u32 {
    num / 5
}

// early return needs ";"
fn devide_by_num(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    50 / num
}

fn main() {
    let formal = "Good bye.";
    let casual = "See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 20;
    println!("{}", devide_by_5(num));
    println!("{}", devide_by_5(50));

    println!("{}", devide_by_num(0));
    println!("{}", devide_by_num(num));
}
