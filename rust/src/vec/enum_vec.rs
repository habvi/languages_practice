#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut row = vec![
        Cell::Int(5),
        Cell::Float(2.3),
        Cell::Text("abcd".to_string()),
    ];
    row.push(Cell::Int(10));
    println!("{:?}", row);
}
