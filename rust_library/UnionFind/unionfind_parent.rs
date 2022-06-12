struct UnionFind {
    parent: Vec<usize>,
    v_size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            v_size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            return false;
        }
        if self.v_size[x] < self.v_size[y] {
            swap(&mut x, &mut y);
        }
        self.v_size[x] += self.v_size[y];
        self.parent[y] = x;
        true
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn group_size(&mut self, x: usize) -> usize {
        let tmp_root: usize = self.root(x);
        self.v_size[tmp_root]
    }
}