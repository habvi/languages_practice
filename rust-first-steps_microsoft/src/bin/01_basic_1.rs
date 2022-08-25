fn main() {
    // todo macro -> panic message
    // todo!("not yet");

    print!("hello\n");
    println!("alphabet : {} to {}", 'A', 'Z');

    // error
    eprint!("hello ");
    eprintln!("world");

    let a;
    a = 5;
    let num = 10;
    let word = "abc";
    println!("{} {} {}", a, num, word);
    println!("{1} {0} {1} {2}", a, num, word);

    println!("{name} {num}", name = "ab", num = 256);
    println!("-----");

    // mut : mutable
    let mut b = 10;
    println!("{}", b);
    b = 20;
    println!("{}", b);
    println!("-----");

    // shadow binding
    let shadow_num = 8;
    println!("{}", shadow_num);
    {
        let shadow_num = shadow_num + 5;
        println!("{}", shadow_num);
    }
    let shadow_num = shadow_num * 3;
    println!("{}", shadow_num);
    println!("-----");

    // int : signed   (i8, i16, i32(default), i64, i128, isize)
    //       unsigned (u8, ...) only >= 0
    {
        let num_32 = 12; // i32
        let num_16: u16 = 14;
        // let num: u32 = "14";   error
        println!("{} {} {}", 5 - num_32, 3i32 + num_32, 5u16 * num_16);
    }

    // tuple
    {
        let tp = ('T', 16i32, true);
        println!("{} {} {}", tp.0, tp.1, tp.2);

        let t: (i32, i32) = (2, 5);
        println!("{:?} {} {}", t, t.0, t.1);

        let (t1, t2): (i32, i32) = t;
        println!("{:?} {} {}", t, t1, t2);
    }
}
