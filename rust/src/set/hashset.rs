use std::collections::HashSet;

#[allow(unused_assignments)]
fn main() {
    {
        let mut st: HashSet<usize> = HashSet::new();
        st.insert(9);
        st.insert(13);

        if st.contains(&9) {
            st.remove(&9);
        }

        // new
        st = HashSet::from([4, 7]);

        let v: Vec<usize> = vec![2, 6, 3, 3, 3];
        // Vec -> HashSet
        st = HashSet::from_iter(v.iter().cloned());

        let v2: Vec<usize> = vec![2, 2, 2, 5, 5, 8, 2];
        st = v2.iter().cloned().collect();
        for x in &st {
            print!("{} ", x);
        }
        println!("");

        // HashSet -> Vec
        let mut v: Vec<_> = st.iter().collect();
        v.sort();
        assert_eq!(v, [&2, &5, &8]);
    }
    {
        #[derive(Eq, Hash, PartialEq, Debug)]
        struct Cat {
            name: String,
            age: usize,
        }

        let mut st: HashSet<Cat> = HashSet::new();
        st.insert(Cat { name: "tama".to_string(), age: 3 });
        st.insert(Cat { name: "nyan".to_string(), age: 2 });
        st.insert(Cat { name: "shiro".to_string(), age: 5 });
        for info in &st {
            println!("{:?} {}", info, info.name);
        }
    }
}
