fn main() {
    // array : same type, fixed size

    // compiler infers length = 3
    let array = ["one", "two", "three"];
    // [T : initialize all value, size : length]
    let mut a2 = [0; 5];
    a2[2] = 10;
    println!("{}", a2[2]);
    println!("-----");


    // vector : multiple types of value, variable size
    let v1 = vec![2, 7, 14, 6];
    println!("{:?}", v1);

    let v2 = vec![0; 8];
    println!("{:?}", v2);
    println!("-----");

    // <vecter><T>
    let mut v3 = Vec::new();
    // type changes from generic 'T' to String
    v3.push("abc");
    v3.push("67");
    // v3.push(3);            error
    println!("{:?}", v3);
    println!("{:?}", v3.pop());
    println!("{:?}", v3);

    let mut v4 = vec![18, 3, 55];
    let x = v4[2];
    println!("{:?} {}", v4, x);
    v4[0] = 4;
    v4[1] += 20;
    println!("{:?}", v4);
    println!("-----");
}