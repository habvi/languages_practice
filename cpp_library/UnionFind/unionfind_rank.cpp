struct UnionFind {
    vector<int> rank;
    UnionFind(int n) : rank(n, -1) { }

    int root(int x) {
        if (rank[x] < 0) {
           return x;
        }
        return rank[x] = root(rank[x]);
    }
    bool unite(int cx, int cy) {
        int x = root(cx), y = root(cy);
        if (x == y) {
            return false;
        }
        if (rank[x] > rank[y]) {
            swap(x, y);
        }
        rank[x] += rank[y];
        rank[y] = x;
        return true;
    }
    bool is_same(int x, int y) {
        return root(x) == root(y);
    }
    int st_size(int x) {
        return -rank[root(x)];
    }
};