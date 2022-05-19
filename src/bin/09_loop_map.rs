fn main() {
    // loop (inf)
    {
        loop {
            println!("hello");
            break;
        }

        let mut counter = 1;
        let count_loop = loop {
            counter *= 2;
            if counter > 100 {
                // stop loop and return counter value
                break counter;
            }
        };
        println!("{}", count_loop);
    }

    // while
    {
        let mut counter = 124;
        while counter > 100 {
            counter -= 1;
        }
        println!("{}", counter);
        println!("-----");
    }

    // for
    {
        let big_birds = ["ostrich", "peacock", "stork"];
        for bird in big_birds.iter() {
            println!("{}", bird);
        }

        for num in 0..3 {
            println!("{}", num);
        }
        println!("-----");

        for num in 0..=3 {
            println!("{}", num * 2);
        }
        println!("-----");
    }
    // Vec
    {
        let v: Vec<i32> = vec![6, 7, 8];
        for i in v.iter() {
            println!("{}", i);
        }

        for i in &v {
            println!("{}", i);
        }
        println!("{:?}", v);

        let mut v: Vec<usize> = vec![1, 2, 3, 4];
        for x in v.iter_mut() {
            *x += 5;
        }
        println!("{:?}", v);

        let v: &mut [i32; 3] = &mut [2, 8, 18];
        for x in v.iter_mut() {
            *x *= 5;
        }
        println!("{:?}", v);
        println!("-----");
    }

    // map
    {
        let v: Vec<usize> = vec![2, 4, 8, 456, 9];

        let doubled: Vec<usize> = v.iter().map(|&e| e * 2).collect();
        println!("{:?} {:?}", v, doubled);

        let plus_5 = v.iter().map(|&e| e + 5).collect::<Vec<usize>>();
        println!("{:?} {:?}", v, plus_5);

        let chars: [char; 5] = ['g', 'd', 'k', 'k', 'n'];
        let hello: String = chars
            .iter()
            .map(|&c| c as u8)
            .map(|x| (x + 1) as char)
            .collect();
        println!("{:?} {}", chars, hello);
    }
}
