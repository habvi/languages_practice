fn main() {
    // todo macro -> panic message
    // todo!("not yet");

    println!("hello");
    println!("alphabet : {} to {}", 'A', 'Z');

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
    let num_32 = 12;      // i32
    let num_16: u16 = 14;
    // let num: u32 = "14";   error
    println!("{} {} {}", 5 - num_32, 3i32 + num_32, 5u16 * num_16);

    // float : f32, f64(default)
    let num_64 = 4.0;  // f64
    let num_32: f32 = 7.0;
    println!("{} {}", num_64 * 2.6, num_32 / 2.0);

    println!("{} {} {}", 9 / 2, 9f64 / 2.0, 9.0 / 2.0);
    println!("-----");

    // bool
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);

    // char : 21bit integer?, string :  &str or String
    let c1 = 'A';
    let c2 = 'f';
    let c3: char = 'S';
    let s1 = "abc";
    let s2: &str = "rrr";
    println!("{} {} {} {} {}", c1, c2, c3, s1, s2);
    println!("-----");

    // tuple
    let tp = ('T', 16i32, true);
    println!("{} {} {}", tp.0, tp.1, tp.2);

    // struct
    // classic struct with named fields (no need ";")
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    // tuple with data types only
    struct Grades(char, char, char, f32);
    // unit struct
    struct Unit;

    // instantiate (&str -> String)
    let user1 = Student {
        name: String::from("aiueo"),
        remote: true,
        level: 4
    };
    let mark1 = Grades('B', 'A', 'A', 3.88);
    println!(
        "{} level: {}, remote: {}, grades: {}",
        user1.name, user1.level, user1.remote, mark1.3
    );

    let user2 = Student {
        level: 2,
        name: String::from("oaiue"),
        remote: false
    };
    let mark2 = Grades('A', 'C', 'B', 3.25);
    println!(
        "{} level: {}, remote: {}, grades: {}",
        user2.name, user2.level, user2.remote, mark2.3
    );
    println!("-----");


    // enum (unit, tuple, classic struct)
    {
        enum WebEvent {
            WELoad,
            WEKeys(String, char),
            WEClick { x: i64, y: i64 },
        }
    }

    // define to use the data from the new structs
    #[derive(Debug)]
    struct KeyPress(String, char);
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64
    }
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    // instantiate and bind the values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}\n", click.x, click.y);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("Keys pressed: {}{}\n", keys.0, keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    // {:#?} needs #[derive(Debug)]
    println!(
        "WebEvent enum structure:\n{:#?}\n{:#?}\n{:#?}\n",
        we_load, we_click, we_key
    );
}