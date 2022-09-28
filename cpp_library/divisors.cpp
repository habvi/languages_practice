vector<long long> div_list(long long x) {
    vector<long long> l, r;
    long long i = 1;
    while (i * i <= x) {
        if (x % i == 0) {
            l.push_back(i);
            if (i != x / i) {
                r.push_back(x / i);
            }
        }
        i++;
    }
    reverse(r.begin(), r.end());
    l.insert(l.end(), r.begin(), r.end());
    return l;
}
