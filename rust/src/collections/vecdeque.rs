use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_front(5);
    que.push_back(8);
    que.push_back(2);
    assert_eq!(que, [5, 8, 2]);

    que = VecDeque::from(vec![2, 5, 9]);
    que.pop_front();
    que.pop_back();
    assert_eq!(que, [5]);

    // from_iter()
    que = VecDeque::from_iter(3..6);
    assert_eq!(que, [3, 4, 5]);

    que = (1..4).collect();
    assert_eq!(que, [1, 2, 3]);

    // VecDeque -> Vec
    let v: Vec<usize> = Vec::from(que);
    assert_eq!(v, [1, 2, 3]);
}
