fn main() {
    // todo macro -> panic message
    // todo!("not yet");

    println!("hello");
    println!("alphabet : {} to {}", 'A', 'Z');

    // error
    eprint!("hello ");
    eprintln!("world");

    let a;
    a = 5;
    let num = 10;
    let word = "abc";
    println!("{} {} {}", a, num, word);
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
        let num_32 = 12;      // i32
        let num_16: u16 = 14;
        // let num: u32 = "14";   error
        println!("{} {} {}", 5 - num_32, 3i32 + num_32, 5u16 * num_16);
    }
    {
        let x;
        let y;
        x = 2147483647;
        y = -2147483648;
        println!("i32: {} ~ {}", y, x);
    }
    {
        let x: i32 = 1_000_000_007;
        println!("{}", x);
    }
    {
        let x: i64 = 99826354728;
        let y: i64 = 99826354728i64;
        let z: i64 = 99826354728_i64;
        println!("{} {} {}", x, y, z);
    }

    // float : f32, f64(default)
    {
        let num_64 = 4.0;       // f64
        let num_32: f32 = 7.0;
        println!("{} {}", num_64 * 2.6, num_32 / 2.0);

        println!("{} {} {}", 9 / 2, 9f64 / 2.0, 9.0 / 2.0);

        let x = 10.;
        let y = 2.5;
        let z: f64 = 6.02e+23;
        let x2: f32 = 10_f32;
        let y2 = 2.5_f32;
        let pi = std::f64::consts::PI;
        println!("{} {} {} {} {}", x + y, z, x2, y2, pi);
        println!("-----");
    }

    // bool
    {
        let is_bigger = 1 > 4;
        let mut a = true;
        a = !a;
        println!("{} {}", is_bigger, a);
    }

    // char : 21bit integer?, string :  &str or String
    {
        let c1 = 'A';
        let c2 = 'f';
        let c3: char = 'S';
        let s1 = "abc";
        let s2: &str = "rrr";
        println!("{} {} {} {} {}", c1, c2, c3, s1, s2);
        println!("-----");
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