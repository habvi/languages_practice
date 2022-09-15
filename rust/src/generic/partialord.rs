// return &T, no need Copy
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn run1() {
    let list = vec![10, 5, 32, 70];
    let result = largest(&list);
    assert_eq!(result, 70);

    let list = vec!['g', 'b', 'z'];
    let result = largest(&list);
    assert_eq!(result, 'z');
}

// -----------------------------------
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest num : {}", self.x);
        } else {
            println!("largest num : {}", self.y);
        }
    }
}

fn run2() {
    let p = Pair::new(5, 8);
    p.cmp_display();
}

// -----------------------------------
fn main() {
    run1();
    run2();
}