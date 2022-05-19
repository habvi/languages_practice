// iterator
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(3);
    println!("{:#?}", counter);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    println!("{:#?}", counter);
    println!("{:?}", counter.next());

    for num in Counter::new(5) {
        println!("{}", num);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    println!("{}", sum_until_10);

    let power_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    println!("{:?}", power_of_2);
}
