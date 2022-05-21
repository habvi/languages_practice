mod authentication_23;

fn main() {
    let mut user = authentication_23::User::new("Alice", "secret");
    println!("{:?}", user);
    // error : can't access. private field
    // println!("{} {}", user.username, user.password_hash);

    println!("{}", user.get_username());

    user.set_password("even-more-secret");
    println!("{:?}", user);

    user = authentication_23::User::new("Bob", "password");
    println!("{:?}", user);
    user.set_password("change-secret");
    println!("{:?}", user);
}
