fn main() {
    // array : same type, fixed size
    {
        // compiler infers length = 3
        let array = ["one", "two", "three"];
        println!("{:?} {}", array, array.len());

        // [T : initialize all value, size : length]
        let mut a = [0; 5];
        a[2] = 10;
        println!("{:?} {} {}", a, a[2], a.len());

        let b: [i32; 4] = [3, 6, 2, 48];
        println!("{:?} {}", b, b.len());
        // allocated
        println!("{}", std::mem::size_of_val(&b));
        println!("-----");
    }

    // Vec : multiple types of value, variable size
    {
        let v: Vec<i32> = vec![2, 7, 14, 6];
        println!("{:?}", v);

        let v: Vec<i32> = vec![0; 8];
        println!("{:?}", v);

        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(2);
        println!("{:?}", v);

        v.extend([6, 7, 8].iter().cloned());
        println!("{:?}", v);

        v.pop();
        println!("{:?}", v);
        println!("-----");
    }
    // Vec<T>
    {
        let mut v = Vec::new();
        // type changes from generic 'T' to String
        v.push("abc");
        v.push("67");
        // v3.push(3);            error
        println!("{:?}", v);
        println!("{:?}", v.pop());
        println!("{:?}", v);
        println!("{}", v.pop().unwrap());
    }

    // count number of the element
    {
        let a = [1, 1, 1, 5, 5];
        let x = 5;
        let num = a.iter().filter(|&e| *e == x).count();
        println!("{}", num);
    }

    // range
    {
        let v: Vec<i32> = (1..=5).collect();
        println!("{:?} {}", v, v.len());
        println!("-----");
    }

    // slice
    {
        let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let b: &[i32] = &a;
        println!("{:?} {:?}", a, b);

        let v: Vec<i32> = vec![6, 7, 8, 9];
        let v2: &[i32] = &v;
        println!("{:?} {:?}", v, v2);

        let c: &[i32] = &a[0..=3];
        let d: &[i32] = &v[1..3];
        let e: &[i32] = &v[2..];
        let f: &[i32] = &v[..2];
        println!("{:?} {:?} {:?} {:?}", c, d, e, f);
        println!("-----");

        let v = [10, 40, 30];
        assert_eq!(Some(&40), v.get(1));
        assert_eq!(Some(&[10, 40][..]), v.get(0..2));
        assert_eq!(None, v.get(3));
        assert_eq!(None, v.get(0..4));
        println!("-----");
    }

    // like stack
    {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(6);
        stack.push(35);
        stack.push(4);
        stack.push(10);
        println!("{:?}", stack);

        if let Some(x) = stack.pop() {
            println!("{}", x);
        }

        while let Some(x) = stack.pop() {
            println!("{}", x);
        }
        println!("-----");
    }

    // sort
    {
        use std::cmp::Reverse;
        let mut v: Vec<usize> = vec![6, 2, 63, 1, 223];
        v.reverse();
        println!("{:?}", v);

        v.sort();
        println!("{:?}", v);

        v.sort_by_key(|&x| Reverse(x));
        println!("{:?}", v);
        println!("-----");
    }

    // change size
    {
        let mut v: Vec<usize> = vec![1, 2, 4, 7];
        v.insert(2, 4);
        v.append(&mut vec![9]);
        println!("{:?}", v);

        v.remove(1);
        println!("{:?}", v);

        let mut tmp: Vec<usize> = v.clone();
        tmp.retain(|&x| x == 4);
        println!("{:?} {:?}", tmp, v);

        v.drain(1..3);
        println!("{:?}", v);
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
    }

    // String -> Vec<char>
    {
        let s: String = String::from("hello");
        let vc: Vec<char> = s.chars().collect();
        println!("{:?}", vc);
    }
}
