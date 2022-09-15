#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn dist_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
fn run1() {
    let int = Point { x: 5, y: 8 };
    println!("{} {}", int.x, int.y);
    assert_eq!(int.x(), &5);

    let float = Point { x: 2.0, y: 7.5 };
    println!("{:?} {} {}", float, float.x(), float.dist_from_origin());
}

// ---------------------------------------------
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
fn run2() {
    let int = Point2 { x: 5, y: 8 };
    println!("{}  {}", int.x, int.y);

    let int_float = Point2 { x: 2, y: 7.5 };
    println!("{:?}", int_float);

    let str_char = Point2 { x: "abc", y: 'd' };
    println!("{:?}", int_float.mixup(str_char));
}

// ---------------------------------------------
fn main() {
    run1();
    run2();
}
