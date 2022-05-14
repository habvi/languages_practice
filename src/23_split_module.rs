mod authentication;

fn main() {
    let mut user = authentication::User::new("Alice", "secret");
    println!("{:?}", user);
    // error : can't access. private field
    // println!("{} {}", user.username, user.password_hash);

    println!("{}", user.get_username());

    user.set_password("even-more-secret");
    println!("{:?}", user);

    user = authentication::User::new("Bob", "password");
    println!("{:?}", user);
    user.set_password("change-secret");
    println!("{:?}", user);
}