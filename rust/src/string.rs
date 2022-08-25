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
        let str = "abc"; // &str
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
        assert_eq!(s.capacity(), 16); // (?)
        assert_eq!(s.contains("world"), true);

        // replace
        s = s.replace("xxx", "hello ");
        assert_eq!(s, "hello world~!");

        // slice
        let str: &str = &s[6..6 + 5]; // world
        assert_eq!(str, "world");
    }
    // String -> Vec<char>
    {
        let s: String = String::from("hello");
        let vc: Vec<char> = s.chars().collect();
        assert_eq!(vc, ['h', 'e', 'l', 'l', 'o']);
    }
}
