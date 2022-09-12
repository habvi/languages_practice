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

fn main() {
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
