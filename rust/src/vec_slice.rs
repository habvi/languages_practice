fn first_word(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}

#[allow(unused_variables, unused_assignments)]
fn main() {
    {
        let v = vec![1, 2, 3, 4, 5];

        // all
        let mut v2: &[i32] = &v;
        v2 = &v[..];
        // v2 : v's start index pointer & len
        v2 = &v[0..=3];
        v2 = &v[1..3];
        v2 = &v[2..];
        v2 = &v[..2];
    }
    {
        let mut s = String::from("abc defg");
        assert_eq!("abc", first_word(&s));
        // immutable borrow
        let w = first_word(&s);
        // mutable borrow
        s.push_str("zzz");
        // error! no more use.
        // println!("{}", w);

        let s = "abc defg";
        assert_eq!("abc", first_word(s));
    }
}
