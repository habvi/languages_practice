#include <bits/stdc++.h>
using namespace std;
#define _GLIBCXX_DEBUG
#define rep(i, a, b) for (int i = a; i < b; ++i)

void line();


int main(){
    vector<string> S = {"abc", "ddddd", "tyugh", "zz", "abc"};

    map<string, int> mp;
    for (auto s : S) {
        mp[s]++;
    }

    // lexicographical ascending order
    for (auto itr = mp.begin(); itr != mp.end(); itr++) {
        cout << itr->first << " : " << itr->second << "\n";
    }

    // same
    for (auto x : mp) {
        cout << x.first << " : " << x.second << "\n";
    }

    // same. structured bindings
    for (auto [s, num] : mp) {
        cout << s << " : " << num << "\n";
    }

    return 0;
}


void line() {
    cout << "------" << endl;
}