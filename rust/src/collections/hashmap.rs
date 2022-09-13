use std::collections::HashMap;

fn main() {
    {
        let mut colors: HashMap<String, String> = HashMap::new();

        colors.insert(String::from("Apple"), String::from("Red"));
        colors.insert(String::from("Lemon"), String::from("Yellow"));

        let stuff: &str = "Lemon";
        // return Option<&Value>
        assert_eq!(colors.get(stuff), Some(&"Yellow".to_string()));

        let rm: &str = "Apple";
        colors.remove(rm);
        // if key not in keys, return None
        assert_eq!(colors.get(rm), None);

        if colors.contains_key(&"Apple".to_string()) {
            println!("unreachable");
        }

        for (fruit, color) in &colors {
            println!("{} : {}", fruit, color);
        }
        println!("-----");
    }
    {
        let mut mp: HashMap<&str, usize> = HashMap::from([("abc", 6), ("def", 80)]);

        // insert only if it doesn't already exist
        // or_insert() -> &mut V
        mp.entry("zzz").or_insert(30);
        mp.entry("zzz").or_insert(0);
        assert_eq!(mp["zzz"], 30);

        *mp.entry("zzz").or_insert(0) += 100;
        assert_eq!(mp["zzz"], 130);

        *mp.get_mut("abc").unwrap() = 0;
        assert_eq!(mp["abc"], 0);

        mp.remove("abc");
        assert_eq!(mp.get("abc"), None);

        // update value
        let value: &mut usize = mp.entry("xx").or_insert(7);
        *value += 1;
        assert_eq!(mp["xx"], 8);
    }
    // zip
    {
        let mp: HashMap<char, usize> = ('a'..='z').zip(0..).collect();
        println!("{:?}", mp);

        let ks = vec!["ab", "cd", "efg"];
        let vs = vec![5, 8, 10];
        let mp: HashMap<_, _> = ks.iter().zip(vs.iter()).collect();
        println!("{:?}", mp);
    }
}
