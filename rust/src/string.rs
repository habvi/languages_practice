#[allow(unused_variables)]
fn main() {
    // char : 4 bytes
    {
        let c = 'a';
        let c: char = 'E';
        assert_eq!(c as u8 - 'A' as u8, 4);
    }
    // string : &str or String
    {
        let str = "abc"; // &str (immutable referance)
        let str: &str = "hello";

        let mut s: String;
        s = str.to_string() + "!!";
        assert_eq!(s, "hello!!");

        s = String::from("xxxworld");
        // add
        s += "~!";
        s.push('a');
        s.push_str("bcd"); // xxxworld~!abcd
        // pop
        for _ in 0..4 {
            s.pop();
        }
        assert_eq!(s.len(), 10);
        assert_eq!(s.is_empty(), false);
        // assert_eq!(s.capacity(), 16);
        assert_eq!(s.contains("world"), true);

        // replace
        s = s.replace("xxx", "hello ");
        assert_eq!(s, "hello world~!");

        // slice: only use a char boundary
        let str: &str = &s[6..6 + 5]; // world
        assert_eq!(str, "world");
    }
    // String -> Vec<char>
    {
        let s: String = String::from("hello");
        let vc: Vec<char> = s.chars().collect();
        assert_eq!(vc, ['h', 'e', 'l', 'l', 'o']);
    }
    // Vec<char> -> String
    {
        let vc: Vec<char> = vec!['a', 'b', 'c'];
        let s: String = vc.iter().collect();
        assert_eq!(s, "abc");
        let s = vc.iter().rev().collect::<String>();
        assert_eq!(s, "cba");
    }
    // format
    {
        let s1 = String::from("ab");
        let s2 = String::from("cd");
        let s3 = String::from("ef");
        let s = format!("{}-{}-{}", s1, s2, s3);
        assert_eq!(s, "ab-cd-ef");
    }
    // add
    {
        let s1 = String::from("abc");
        let s2 = String::from("def");
        // only add &str to String
        // s1 move
        // &String -> &str : deref coercion (&s2 -> &s2[..])
        let s = s1 + &s2;
    }
}
