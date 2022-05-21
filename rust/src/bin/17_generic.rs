#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug, PartialEq)]
struct Point2<T, U> {
    x: T,
    y: U,
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
}

use std::fmt;

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 6 };
    let float = Point { x: 3.6, y: 9.2 };
    let string_slice = Point {
        x: "high",
        y: "low",
    };
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}",
        boolean, integer, float, string_slice
    );

    let int_bool = Point2 { x: 8, y: false };
    // Point2<f64, &'static str>
    let float_string = Point2 { x: 2.8, y: "hello" };
    println!("{:?}\n{:?}", int_bool, float_string);

    // shared behavior
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    println!("{} {}", circle.area(), rectangle.area());

    let p1 = Point3 { x: 1, y: 4 };
    let p2 = Point3 { x: 10, y: -5 };
    // can't compare
    if p1 == p2 {
        println!("eqaul");
    } else {
        println!("not eqaul");
    }
    println!("{:?} {:?}", p1, p2);

    // add : fmt::Display
    println!("{} {}", p1, p2);
}
