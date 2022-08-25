#[allow(unused_variables, unused_assignments)]
fn main() {
    let mut v: Vec<i32> = vec![0; 8];

    v = vec![1, 2, 3];
    println!("{:?}", v);

    v = Vec::new();
    v.push(5);
    v.extend([5, 6, 7, 8, 9].iter().cloned());
    v.pop();
    assert_eq!(vec![5, 5, 6, 7, 8], v);

    // Vec<T>
    {
        let mut v = Vec::new(); // Vec<{unknown}>
        v.push("abc"); // Vec<&str>
        v.push("67");
        println!("{:?}", v.pop()); // Some("67")
        println!("{}", v.pop().unwrap()); // abc
    }
    // count
    {
        let num = v.iter().filter(|&e| *e == 5).count();
        assert_eq!(num, 2);
    }
    // range
    {
        let v: Vec<i32> = (1..=4).collect();
        assert_eq!(vec![1, 2, 3, 4], v);
    }
    // slice
    {
        let mut v2: &[i32] = &v; // all
        v2 = &v[0..=3];
        v2 = &v[1..3];
        v2 = &v[2..];
        v2 = &v[..2];
    }
    // get(i) -> Some() or None
    {
        let v = vec![10, 40, 30];
        assert_eq!(v.get(1), Some(&40));
        assert_eq!(v.get(3), None);
        assert_eq!(v.get(0..4), None);
        assert_eq!(v.get(0..3), Some(&[5, 5, 6][..]));
    }
    // like stack
    {
        let mut stack: Vec<i32> = Vec::new();
        if let None = stack.pop() {
            println!("None!!");
        }
        stack.push(4);
        stack.push(-5);
        stack.push(10);
        if let Some(x) = stack.pop() {
            println!("let : {}", x);
        }
        while let Some(x) = stack.pop() {
            println!("while : {}", x);
        }
    }
    // sort
    {
        let mut v: Vec<usize> = vec![5, 1, 3, 4];
        v.reverse();
        assert_eq!(vec![4, 3, 1, 5], v);

        let v2: Vec<usize> = v.iter().rev().cloned().collect();
        assert_eq!(vec![5, 1, 3, 4], v2);

        v.sort();
        assert_eq!(vec![1, 3, 4, 5], v);

        use std::cmp::Reverse;
        v.sort_by_key(|&x| Reverse(x));
        assert_eq!(vec![5, 4, 3, 1], v);
    }
    // sum
    {
        let total: i32 = v.iter().sum();
        assert_eq!(31, total);
    }
    // split
    {

    }
    // change size
    // insert, append, remove, extend, chain, retain, drain
    {
        let mut v: Vec<usize> = vec![1, 4, 2];
        v.insert(2, 10);
        v.append(&mut vec![9, 8]);
        assert_eq!(v, vec![1, 4, 10, 2, 9, 8]);

        v.remove(2);
        assert_eq!(v, vec![1, 4, 2, 9, 8]);

        let mut tmp: Vec<usize> = v.clone();
        tmp.retain(|&x| x % 2 == 0);
        assert_eq!(tmp, vec![4, 2, 8]);

        v.drain(1..3);
        assert_eq!(v, vec![1, 9, 8]);
    }
    // swap
    {
        // use std::mem::swap;
    }
}
