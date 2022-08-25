#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }
    (Age::New, miles)
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!(
                "Used car: {:?}, {}, Hard Top, {} miles",
                motor, color, miles
            )
        } else {
            println!(
                "Used car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            )
        }
    } else {
        if roof {
            println!("New car: {:?}, {}, Hard Top, {} miles", motor, color, miles)
        } else {
            println!(
                "New car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            )
        }
    }

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let colors = ["Blue", "Red", "White"];

    let mut engine = Transmission::Manual;
    car_factory(String::from(colors[1]), engine, true, 0);

    engine = Transmission::Automatic;
    car_factory(String::from(colors[2]), engine, false, 100);

    engine = Transmission::SemiAuto;
    car_factory(String::from(colors[0]), engine, false, 200);
}
