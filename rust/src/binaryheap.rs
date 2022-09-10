use std::collections::BinaryHeap;

fn main() {
    let mut hq: BinaryHeap<i64> = BinaryHeap::new();

    hq.push(5);
    hq.push(1);
    hq.push(7);
    assert_eq!(Some(7), hq.pop());  // largest num

    println!("{}", hq.pop().unwrap());

    if let Some(x) = hq.pop() {
        println!("{}", x);
    } else {
        println!("empty..");
    }

    match hq.pop() {
        Some(x) => println!("{}", x),
        None => println!("empty~"),
    }
}
