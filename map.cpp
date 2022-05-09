#include <bits/stdc++.h>
using namespace std;
#define _GLIBCXX_DEBUG

void line();


int main(){
    vector<string> S = {"abc", "ddddd", "tyugh", "zz", "abc"};

    // get value by key : O(logN)
    // unordered_map(hash) : O(1)

    // map<key, value>
    map<string, int> mp;
    for (auto s : S) {
        mp[s]++;
    }

    // lexicographical ascending order
    for (auto itr = mp.begin(); itr != mp.end(); ++itr) {
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