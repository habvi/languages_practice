#[allow(dead_code)]
mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub bread: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn order(bread: &str) -> Breakfast {
            Breakfast {
                bread: String::from(bread),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::back_of_house::Breakfast::order("Danish");
    // relative path
    back_of_house::Breakfast::order("Danish");

    // struct pub -> decide on each
    let mut meal = back_of_house::Breakfast::order("toast");
    println!("{:?}", meal);
    meal.bread = String::from("croissant");
    println!("{:?}", meal);
    // error. private field.
    // meal.seasonal_fruit = String::from("grape");

    // enum pub -> all public
    let appetizer = back_of_house::Appetizer::Salad;
    println!("{:?}", appetizer);
}

fn main() {
    eat_at_restaurant();
}
