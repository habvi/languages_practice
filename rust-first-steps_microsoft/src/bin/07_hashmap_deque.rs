use std::collections::HashMap;

fn main() {
    let mut colors: HashMap<String, String> = HashMap::new();

    colors.insert(String::from("Apple"), String::from("Red"));
    colors.insert(String::from("Lemon"), String::from("Yellow"));

    let stuff: &str = "Lemon";
    // return Option<&Value>. Wrapped by "Some()"
    println!("{} : {:?}", stuff, colors.get(stuff));

    let rm: &str = "Apple";
    colors.remove(rm);
    println!("removed {}", rm);
    // if key not in keys, return None
    println!("{:?}", colors.get(rm));

    colors.insert("Orange".to_string(), "Orange".to_string());
    if colors.contains_key(&"Apple".to_string()) {
        println!("unreachable");
    }

    // if not found -> panic
    println!("{}", colors["Lemon"]);

    for (fruit, color) in &colors {
        println!("{fruit}: {color}");
    }
    println!("-----");
}
