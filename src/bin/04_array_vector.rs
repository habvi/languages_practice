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
    }
}