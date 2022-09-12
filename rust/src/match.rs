#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("from {:?}, ", state);
            25
        },
    }
}

fn main() {
    println!("{:?} cent", value_in_cents(&Coin::Nickel));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{:?} cent", value_in_cents(&coin));

    if let Coin::Quarter(state) = coin {
        println!("{:?}", state);
    } else {
        println!("not quarter.");
    }
}
