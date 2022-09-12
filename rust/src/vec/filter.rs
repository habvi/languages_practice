fn main() {
    let v: Vec<_> = (0..10_i64).filter(|&x| x % 2 == 0).collect();
    println!("{:?}", v);

    let v: Vec<_> = (0..10_i64).map(|x| x * 2).collect();
    println!("{:?}", v);

    let v: Vec<_> = (0..10_i64).filter(|&x| x % 2 == 0).map(|x| x * 2).collect();
    println!("{:?}", v);

    let v: Vec<usize> = (0..10).filter(|&x| x % 2 == 0).map(|x| (x as usize).pow(2)).collect();
    println!("{:?}", v);

    let v: Vec<usize> = (0..10).collect();
    let x = v.iter().filter(|&x| x % 2 == 0).count();
    println!("{}", x);
}
