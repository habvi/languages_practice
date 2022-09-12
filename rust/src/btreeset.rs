use std::collections::BTreeSet;

// want to do: -> struct, impl

// the largest element < x, or None
fn lt(s: &BTreeSet<i64>, x: i64) -> i64 {
    match s.range(..x).next_back() {
        None => -1,
        Some(&num) => num,
    }
}
// the largest element <= x, or None
fn le(s: &BTreeSet<i64>, x: i64) -> i64 {
    match s.range(..=x).next_back() {
        None => -1,
        Some(&num) => num,
    }
}
// the smallest element > x, or None
fn gt(s: &BTreeSet<i64>, x: i64) -> i64 {
    match s.range(x + 1..).next() {
        None => -1,
        Some(&num) => num,
    }
}
// the smallest element >= x, or None
fn ge(s: &BTreeSet<i64>, x: i64) -> i64 {
    match s.range(x..).next() {
        None => -1,
        Some(&num) => num,
    }
}

fn run1() {
    let mut st: BTreeSet<i64> = BTreeSet::new();
    st.insert(5);
    st.insert(8);
    st.insert(10);
    let can_remove = st.remove(&3);
    assert_eq!(can_remove, false);
    // {5, 8, 10}

    assert_eq!(lt(&st, 2), -1);
    assert_eq!(lt(&st, 6), 5);
    assert_eq!(le(&st, 8), 8);
    assert_eq!(gt(&st, 10), -1);
    assert_eq!(gt(&st, 9), 10);
    assert_eq!(ge(&st, 2), 5);
    assert_eq!(ge(&st, 10), 10);
}


// -------------------------------------------
fn run2() {
    let mut s = BTreeSet::new();
    s.insert(5);
    s.insert(10);
    s.insert(2);
    s.insert(3);
    // {2, 3, 5, 10}

    let x = 3;
    let lt = *s.range(..x).next_back().unwrap();
    assert_eq!(lt, 2);
    let le = *s.range(..=x).next_back().unwrap();
    assert_eq!(le, 3);
    let gt = *s.range(x + 1..).next().unwrap();
    assert_eq!(gt, 5);
    let ge = *s.range(x..).next().unwrap();
    assert_eq!(ge, 3);

    if let Some(num) = s.range(20..).next() {
        println!("{}", num);
    } else {
        println!("None");
    }

    let mut itr = s.range(10..);
    assert_eq!(itr.next(), Some(&10));
    assert_eq!(itr.next(), None);

    // 3 5
    for i in s.range(3..8) {
        print!("{} ", i);
    }
    println!("");

    // 5 3 2
    for i in s.range(..8).rev() {
        print!("{} ", i);
    }
    println!("");

    // 5 10
    use std::ops::Bound::{Included, Excluded};
    for i in s.range((Excluded(&3), Included(&10))) {
        print!("{} ", i);
    }
    println!("");

}

// -------------------------------------------
fn main() {
    run1();
    run2();
}
