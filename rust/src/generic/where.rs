use std::fmt::{Display, Debug};

fn func1<T: Display + Clone,  U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("{} {:?}", t, u);
    1
}

fn func2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{
    println!("{} {:?}", t, u);
    1
}

fn main() {
    func1(&2, &8);
    println!("{}", func2(&2, &8));
}
