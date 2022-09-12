fn main() {
    let mut stack: Vec<i32> = Vec::new();

    // pop -> Option
    if let None = stack.pop() {
        println!("None!!");
    }

    stack.push(4);
    stack.push(-5);
    stack.push(10);
    stack.push(8);

    if let Some(x) = stack.pop() {
        println!("let : {}", x);
    }

    println!("unwrap : {}", stack.pop().unwrap());

    while let Some(x) = stack.pop() {
        println!("while : {}", x);
    }
}
