fn main() {
    if 1 == 2 {
        println!("equal");
    } else {
        println!("not equal");
    }

    let formal = true;
    let greeting = if formal {
        "Good day to you."       // String
    } else {
        "Hey!"
    };
    println!("{}", greeting);

    let num = 500;
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num > 555 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("{}", out_of_range)
}