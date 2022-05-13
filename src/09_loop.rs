fn main() {
    // loop (inf)
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


    // while
    while counter > 100 {
        counter -= 1;
    }
    println!("{}", counter);
    println!("-----");


    // for
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("{}", bird);
    }

    for num in 0..6 {
        println!("{}", num);
    }
    println!("-----");

    for num in 0..=3 {
        println!("{}", num * 2);
    }
    println!("-----");
}