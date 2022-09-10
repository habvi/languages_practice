fn main() {
    // loop
    {
        let mut i = 0;
        loop {
            if i == 5 {
                break;
            }
            i += 1;
        }
    }
    // let loop
    {
        let mut i = 0;
        let a = loop {
            if i >= 10 {
                break i;
            }
            i += 1;
        };
        assert_eq!(a, 10);
    }
    // loop label
    {
        let mut count = 0;
        'countup_to_3: loop {
            let mut i = 0;
            loop {
                println!("count: {}, i: {}", count, i);
                if count == 3 {
                    break 'countup_to_3;
                }
                if i == 1 {
                    println!("------");
                    break;
                }
                i += 1;
            }
            count += 1;
        }
    }
}
