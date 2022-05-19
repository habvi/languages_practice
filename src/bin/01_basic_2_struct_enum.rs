// struct
// classic struct with named fields (no need ";")
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

// tuple with data types only
struct Grades(char, char, char, f32);

// unit struct
#[allow(dead_code)]
struct Unit;

// -----------------------------------------------
#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    // instantiate (&str -> String)
    let user1 = Student {
        name: String::from("aiueo"),
        remote: true,
        level: 4,
    };
    let mark1 = Grades('B', 'A', 'A', 3.88);
    println!(
        "{} level: {}, remote: {}, grades: {}",
        user1.name, user1.level, user1.remote, mark1.3
    );

    let user2 = Student {
        level: 2,
        name: String::from("oaiue"),
        remote: false,
    };
    let mark2 = Grades('A', 'C', 'B', 3.25);
    println!(
        "{} level: {}, remote: {}, grades: {}",
        user2.name, user2.level, user2.remote, mark2.3
    );
    println!("-----");

    // -----------------------------------------------
    // instantiate and bind the values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("Keys pressed: {}{}", keys.0, keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    // {:#?} needs #[derive(Debug)]
    println!(
        "WebEvent enum structure:\n{:#?}\n{:#?}\n{:#?}\n",
        we_load, we_click, we_key
    );
}
