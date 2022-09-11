// field
#[derive(Debug)]
struct User {
    name: String,
    id: u64,
    active: bool,
}

fn build_user1(name: String, id: u64) -> User {
    User {
        name: name,
        id: id,
        active: true,
    }
}

// field init shorthand syntax
fn build_user2(name: String, id: u64) -> User {
    User {
        name,
        id,
        active: true,
    }
}

fn run1() {
    {
        let mut user = User {
            name: String::from("Alice"),
            id: 1,
            active: true,
        };
        println!("{} {} {}", user.name, user.id, user.active);

        user.active = false;
        println!("{:?}", user);
        println!("-----");
    }
    {
        let mut user1 = build_user1("Alice".to_string(), 1);
        println!("{:?}", user1);
        user1 = build_user2("Bob".to_string(), 2);
        println!("{:?}", user1);

        // struct update syntax
        let user2 = User {
            name: String::from("abc"),
            ..user1
        };
        println!("{:#?}", user2);
    }
}

// ---------------------------------------
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn run2() {
    let rec1 = Rectangle { width: 20, height: 50 };
    assert_eq!(rec1.area(), 1000);

    let rec2 = Rectangle { width: 10, height: 30 };
    assert_eq!(rec1.can_hold(&rec2), true);
    assert_eq!(rec2.can_hold(&rec1), false);

    let rec3 = Rectangle::square(5);
    println!("{:?}", rec3);
}

// ---------------------------------------
fn main() {
    run1();
    println!("-----");

    run2();
}
