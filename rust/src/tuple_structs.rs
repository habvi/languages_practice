#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    println!("{:?}", black);
    let xy = Point(2, 5);
    println!("{:?} {} {}", xy, xy.0, xy.1);
}
