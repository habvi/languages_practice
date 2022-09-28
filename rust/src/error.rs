// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File; // -> Result<std::fs::File, std::io::Error>
use std::io::ErrorKind;

#[allow(dead_code)]
fn run1(){
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("failed to create file: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("no such file: {:?}", error)
        },
    };
    println!("{:?}", f);
}

// ---------------------------------------
#[allow(dead_code)]
fn run2() {
    let f = File::open("hello.txt").unwrap();
    println!("{:?}", f);
}

// ---------------------------------------
#[allow(dead_code)]
fn run3() {
    let f = File::open("hello.txt").expect("no such file");
    println!("{:?}", f);
}

// ---------------------------------------
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn run4(){
    println!("{:?}", read_username_from_file());
}

// ---------------------------------------
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn run5() {
    println!("{:?}", read_username_from_file2());
}

// ---------------------------------------
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn run6() {
    println!("{:?}", read_username_from_file3());
}

// ---------------------------------------
fn main() {
    // run1();
    // run2();
    // run3();
    // run4();
    // run5();
    run6();
}
