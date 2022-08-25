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
        // type changes from generic T to &str
        v.push("abc");
        v.push("67");
        // v3.push(3);            error
        println!("{:?}", v);
        println!("{:?}", v.pop());
        println!("{:?}", v);
        println!("{}", v.pop().unwrap());
    }
}
