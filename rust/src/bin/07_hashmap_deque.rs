use std::collections::HashMap;

fn _hashmap() {
    {
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
    {
        let mut mp: HashMap<&str, usize> = HashMap::from([("abc", 6), ("def", 80)]);
        println!("{:?}", mp);

        // insert only if it doesn't already exist
        mp.entry("zzz").or_insert(30);
        mp.entry("zzz").or_insert(50);
        println!("{:?}", mp);

        // update value
        let value = mp.entry("z").or_insert(7);
        *value += 1;
        println!("{:?}", mp);
    }
}

use std::collections::VecDeque;
use std::iter::FromIterator;

fn _deque() {
    let mut d: VecDeque<usize> = VecDeque::new();
    println!("{:?}", d);
    d.push_front(5);
    d.push_back(8);
    println!("{:?}", d);

    let mut d: VecDeque<usize> = VecDeque::from(vec![2, 5, 9]);
    println!("{:?}", d);
    d.pop_front();
    d.pop_back();
    println!("{:?}", d);

    let d: VecDeque<usize> = VecDeque::from_iter(3..10);
    println!("{:?}", d);

    let d: VecDeque<usize> = (1..4).collect();
    println!("{:?}", d);

    let v: Vec<usize> = Vec::from(d);
    println!("{:?}", v);
}

use std::collections::HashSet;

fn _hashset() {
    {
        let mut set: HashSet<usize> = HashSet::new();
        set.insert(9);
        set.insert(13);
        println!("{:?}", set);

        if set.contains(&9) {
            set.remove(&9);
        }
        println!("{:?}", set);

        set = HashSet::from([4, 7]);
        println!("{:?}", set);

        let v: Vec<usize> = vec![2, 6, 3, 3, 3];
        // Vec -> HashSet
        set = HashSet::from_iter(v.iter().cloned());
        println!("{:?}", set);

        let v2: Vec<usize> = vec![2, 2, 2, 5, 5, 8, 2];
        set = v2.iter().cloned().collect();
        println!("{:?}", set);

        for x in &set {
            println!("{}", x);
        }

        // HashSet -> Vec
        let v: Vec<_> = set.iter().collect();
        println!("{:?}", v);
    }
    {
        #[derive(Eq, Hash, PartialEq, Debug)]
        struct Cat {
            name: String,
            age: usize,
        }

        let mut set: HashSet<Cat> = HashSet::new();
        set.insert(Cat { name: "tama".to_string(), age: 3 });
        set.insert(Cat { name: "nyan".to_string(), age: 2 });
        set.insert(Cat { name: "shiro".to_string(), age: 5 });
        for info in &set {
            println!("{:?}", info);
        }
    }
}

fn main() {
    _hashmap();
    println!("-----");

    _deque();
    println!("-----");

    _hashset();
}
