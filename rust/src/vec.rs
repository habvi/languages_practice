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
    // get(i) -> Some() or None
    {
        let v = vec![10, 40, 30];
        assert_eq!(v.get(1), Some(&40));
        assert_eq!(v.get(3), None);
        assert_eq!(v.get(0..4), None);
        assert_eq!(v.get(0..3), Some(&[5, 5, 6][..]));
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
    // Vec yet
    {
        let v: Vec<i32> = vec![6, 7, 8];
        for i in v.iter() {
            println!("{}", i);
        }

        for i in &v {
            println!("{}", i);
        }
        println!("{:?}", v);

        let mut v: Vec<usize> = vec![1, 2, 3, 4];
        for x in v.iter_mut() {
            *x += 5;
        }
        println!("{:?}", v);

        let v: &mut [i32; 3] = &mut [2, 8, 18];
        for x in v.iter_mut() {
            *x *= 5;
        }
        println!("{:?}", v);
        println!("-----");
    }
}
