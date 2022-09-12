#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rec1 = Rectangle { width: 20, height: 50 };
    assert_eq!(rec1.area(), 1000);

    let rec2 = Rectangle { width: 10, height: 30 };
    assert_eq!(rec1.can_hold(&rec2), true);
    assert_eq!(rec2.can_hold(&rec1), false);

    let rec3 = Rectangle::square(5);
    println!("{:?}", rec3);
}
