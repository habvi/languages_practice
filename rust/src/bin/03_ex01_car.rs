struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}

fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "car 1. {}, {:?}, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::Automatic, true);
    println!(
        "car 2. {}, {:?}, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
}
