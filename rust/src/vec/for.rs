fn main() {
    let v: Vec<i32> = vec![6, 7, 8];

    // &
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    // iter()
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    // &mut
    let mut v: Vec<usize> = vec![1, 2, 3, 4];
    for x in &mut v {
        *x += 2;
    }
    assert_eq!(v, [3, 4, 5, 6]);

    // iter_mut
    for x in v.iter_mut() {
        *x += 5;
    }
    assert_eq!(v, [8, 9, 10, 11]);

    let v: &mut [i32; 3] = &mut [2, 8, 18];
    for x in v.iter_mut() {
        *x *= 5;
    }
    assert_eq!(v, &[10, 40, 90]);
}
