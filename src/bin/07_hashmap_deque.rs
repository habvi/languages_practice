use std::collections::HashMap;

fn _hashmap() {
    // make empty hashmap
    let mut colors: HashMap<String, String> = HashMap::new();

    colors.insert(String::from("Apple"), String::from("Red"));
    colors.insert(String::from("Lemon"), String::from("Yellow"));

    let stuff: &str = "Lemon";
    // return Option<&Value>. Wrapped by "Some()"
    println!("{} : {:?}", stuff, colors.get(stuff));

    let rm: &str = "Apple";
    colors.remove(rm);
    println!("removed {}", rm);
    // if key not in keys, return None
    println!("{:?}", colors.get(rm));
}


use std::collections::VecDeque;
use std::iter::FromIterator;

fn _deque() {
    let mut d: VecDeque<usize> = VecDeque::new();
    println!("{:?}", d);
    d.push_front(5);
    d.push_back(8);
    println!("{:?}", d);

    let mut d: VecDeque<usize> = VecDeque::from(vec![2, 5, 9]);
    println!("{:?}", d);
    d.pop_front();
    d.pop_back();
    println!("{:?}", d);

    let d: VecDeque<usize> = VecDeque::from_iter(3..10);
    println!("{:?}", d);

    let d: VecDeque<usize> = (1..4).collect();
    println!("{:?}", d);

    let v: Vec<usize> = Vec::from(d);
    println!("{:?}", v);
}


fn main() {
    _hashmap();
    _deque();
}