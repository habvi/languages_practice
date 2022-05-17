fn main() {
    {
        // panic!("something wrong...");
        // std::process::exit(1);
    }

    // Option
    {
        let v: Vec<i32> = vec![0, 1, 2];
        // index out of bounds
        // println!("{}", v[3]);

        // enum Option<T> {
        //     None,
        //     Some(T),
        // }
        println!("{:?} {:?}", v.get(2), v.get(5));
        println!("-----");
    }

    // match : must cover all patterns
    {
        let fruits: Vec<&str> = vec!["Apple", "Lemon", "Orange"];
        for &i in [0, 2, 50].iter() {
            match fruits.get(i) {
                Some(&"Orange") => println!("Orange is awesome!"),
                Some(fruit_name) => println!("{}", fruit_name),
                None => println!("There is no fruit!"),
            }
        }
        println!("-----");
    }

    // _ : wild card
    {
        let num: Option<u8> = Some(7);
        match num {
            Some(7) => println!("Lucky number"),
            _ => {},
        }

        let a = [1, 2];
        for i in 0..=1 {
            match i {
                0 => {
                    println!("index 0 is {}", a[i]);
                },
                1 => {
                    println!("index 1 is {}", a[i]);
                },
                _ => unreachable!(),
            }
        }
    }

    // if let
    {
        let num: Option<u8> = Some(7);
        if let Some(7) = num {
            println!("Lucky number");
        }
        println!("-----");
    }


    // unwrap
    {
        let gift: Option<&str> = Some("candy");
        // access to Option value
        assert_eq!(gift.unwrap(), "candy");

        #[allow(unused_variables)]
        let empty_gift: Option<&str> = None;
        // None can't unwrap -> panic
        // assert_eq!(empty_gift.unwrap(), "candy");
        if let Some(stuff) = empty_gift {
            println!("{}", stuff);
        } else {
            println!("empty...");
        }
        println!("-----");
    }


    // expect
    {
        let a = Some("value");
        assert_eq!(a.expect("custom panic massage!"), "value");

        #[allow(unused_variables)]
        let b: Option<&str> = None;
        // panic
        // b.expect("custom panic massage!");
    }

    {
        assert_eq!(Some("dog").unwrap_or("cat"), "dog");
        assert_eq!(None.unwrap_or("cat"), "cat");
    }

    {
        let mut a: Option<u8> = Option::None;
        println!("{:?}", a);
        a = Option::Some(25);
        println!("{:?} {}", a, a.unwrap());

        let mut b: Option<&str> = None;
        println!("{:?}", b);
        b = Some("apple");
        println!("{:?} {}", b, b.unwrap());

        if let Some(x) = b {
            println!("{}", x);
        }
    }
}