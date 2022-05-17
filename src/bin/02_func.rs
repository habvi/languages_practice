fn goodbye(message: &str) {
    println!("{}", message);
}

// no need ";"
fn devide_by_5(num: u32) -> u32 {
    num / 5
}

// early return needs ";"
fn devide_by_num(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    50 / num
}

fn func_1() {
    let formal = "Good bye.";
    let casual = "See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 20;
    println!("{}", devide_by_5(num));
    println!("{}", devide_by_5(50));

    println!("{}", devide_by_num(0));
    println!("{}", devide_by_num(num));
}


// ------------------------------------------------
fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn num_3(x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    (x + y, y + z, z + x)
}

fn print_s(s: &String) -> () {
    println!("{}", s);
}

fn func_2() {
    println!("{}", mul(5, 9));

    let (a, b, c) = num_3(4, 8, 2);
    println!("{} {} {}", a, b, c);

    let s: String = String::from("hello");
    print_s(&s);

    let t = s;
    println!("{} {}", t, &t);
}


// ------------------------------------------------
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn move_to(&self, step: i32) {
        match self {
            Direction::Up => println!("up: {} step", step),
            Direction::Down => println!("down: {} step", step),
            Direction::Right => println!("right: {} step", step),
            Direction::Left => println!("left: {} step", step),
        }
    }
}

fn func_3() {
    let mut direction = Direction::Right;
    let x: i32 = 5;
    direction.move_to(x);

    direction = Direction::Down;
    direction.move_to(8);
}


// ------------------------------------------------
#[allow(dead_code)]
#[derive(Debug)]
enum Action {
    Stop,
    Move { x: i32, y: i32 },
    Say(String, String),
}

fn func_4() {
    let action1: Action = Action::Move { 
        x: 200,
        y: 50
    };
    println!("{:?}", action1);

    if let Action::Move { x: x1, y: y1 } = action1 {
        println!("{} {}", x1, y1);
    }
}


// ------------------------------------------------
#[derive(Debug)]
struct Cat {
    name: String,
    age: i32,
}

impl Cat {
    fn introduce(&self, food: &str) {
        println!("{} ({}) likes {}", &self.name, &self.age, food);
    }
    fn age_increment(&mut self) {
        self.age += 1
    }
}

fn func_5() {
    let mut cat = Cat {
        name: String::from("nyan"),
        age: 5,
    };
    println!("{:?}", cat);
    cat.introduce("DHA");

    cat.name = String::from("tama");
    cat.age_increment();
    println!("{:?}", cat);
    let s = "meat";
    cat.introduce(s);
}


// ------------------------------------------------
trait Animal {
    fn name(&self) -> &String;
    fn age(&self) -> u8;
    fn say(&self);
    fn eat(&self, food: String);
    fn human_age(&self) -> u8;
}

#[derive(Debug)]
struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog {
    fn name(&self) -> &String {
        &self.name
    }
    fn age(&self) -> u8 {
        self.age
    }
    fn say(&self) {
        println!("{}({}) : wa-n", self.name, self.age);
    }
    fn eat(&self, food: String) {
        println!("{}({}) eats {}", self.name, self.age, food);
    }
    fn human_age(&self) -> u8 {
        self.age * 7
    }
}

fn info_dog(animal: &dyn Animal, food: String) {
    animal.say();
    animal.eat(food);
    println!("if human, age is {}.", animal.human_age());
}

fn func_6() {
    // no use info_dog
    // let mut dog = Dog {
    //     name: String::from("pochi"),
    //     age: 3,
    // };
    // println!("{:?}", dog);
    // dog.say();
    // dog.eat(String::from("dog food"));
    // println!("{}", dog.human_age());
    // dog.age += 1;
    // println!("{:?}", dog);
    // println!("if human, age is {}.", dog.human_age());

    let dog = Dog {
        name: String::from("pochi"),
        age: 4,
    };
    info_dog(&dog, String::from("fruit"));
}



// ------------------------------------------------
fn main() {
    func_1();
    println!("-----");

    func_2();
    println!("-----");

    func_3();
    println!("-----");

    func_4();
    println!("-----");

    func_5();
    println!("-----");

    func_6();
}