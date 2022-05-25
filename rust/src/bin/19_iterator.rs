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
    {
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

    // iter(), into_iter() : array
    {
        let arr = [1, 2, 3, 4, 5];
        let mut is_2 = arr.iter().any(|&x| x == 2);
        println!("{:?} {}", arr, is_2);

        is_2 = arr.into_iter().any(|x| x == 2);
        println!("{:?} {}", arr, is_2);
        println!("-----");
    }
    // iter(), into_iter() : vector
    {
        let v: Vec<usize> = vec![1, 2, 3, 4, 5];
        // iter() : reference
        let mut v2: Vec<usize> = v.iter().map(|x| x * 5).collect();
        println!("{:?} {:?}", v, v2);

        // into_iter() : value move
        v2 = v.into_iter().map(|x| x * 5).collect();
        // error
        // println!("{:?}", v);
        println!("{:?}", v2);
        println!("-----");
    }
    // iter_mut()
    {

    }

    // zip (yet)
    {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        let ab = a.iter().zip(b.iter());
        for (a, b) in ab {
            println!("{} {}", a, b);
        }
        println!("-----");
    }

    // map
    {
        let v: Vec<usize> = vec![2, 4, 8, 456, 9];

        let doubled: Vec<usize> = v.iter().map(|x| x * 2).collect();
        println!("{:?} {:?}", v, doubled);

        let plus_5 = v.iter().map(|x| x + 5).collect::<Vec<usize>>();
        println!("{:?} {:?}", v, plus_5);

        let chars: [char; 5] = ['g', 'd', 'k', 'k', 'n'];
        let hello: String = chars
            .iter()
            .map(|&x| x as u8)
            .map(|x| (x + 1) as char)
            .collect();
        println!("{:?} {}", chars, hello);
        println!("-----");
    }

    // filter
    {
        let v: Vec<&str> = vec!["abc", "384", "abc", "a", "abc"];
        let t: &str = "abc";
        let num: usize = v.into_iter().filter(|x| x == &t).count();
        println!("{}", num);

        let vvc: Vec<Vec<char>> = vec![vec!['#', '#', '.'], vec!['.', '#', '#']];
        let total: usize = vvc.iter().map(|vc| vc.iter().filter(|&c| *c == '#').count()).sum();
        println!("{}", total);
        println!("-----");
    }

    // enumerate
    {
        let a = ['a', 'b', 'c'];
        let mut enm = a.iter().enumerate();
        println!("{:?} {:?} {:?}", enm.next(), enm.next(), enm.next());
        println!("-----");
    }

    // filter
    {
        let v: Vec<&str> = vec!["abc", "384", "abc", "a", "abc"];
        let t: &str = "abc";
        let num: usize = v.into_iter().filter(|x| x == &t).count();
        println!("{}", num);

        let vvc: Vec<Vec<char>> = vec![vec!['#', '#', '.'], vec!['.', '#', '#']];
        let total: usize = vvc.iter().map(|vc| vc.iter().filter(|&c| *c == '#').count()).sum();
        println!("{}", total);
        println!("-----");
    }

    // for_each
    {

    }
}
