// in  : [1, 1, 2, 1, 3, 3, 3]
// out : [[1, 1], [2], [1], [3, 3, 3]]

#[derive(Debug)]
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let mut cursor = 1;
        let first = &self.inner[0];
        for elm in &self.inner[1..] {
            if elm == first {
                cursor += 1;
            } else {
                break;
            }
        }

        let items = self.inner.drain(0..cursor).collect();
        Some(items)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|

    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data = vec![1, 1, 2, 1, 3, 3, 3];
    let group = Groups::new(data);
    println!("before: {:?}", group.inner);
    println!("after: {:?}", group.into_iter().collect::<Vec<Vec<_>>>());
}