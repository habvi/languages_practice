#include <bits/stdc++.h>
using namespace std;

void line();
void cout_all(set<int>);


// set
//     rend                rbegin
//       _    2    6    9    13    _
//          begin                 end

int main() {
    set<int> s{13, 6};

    // O(logN)
    s.insert(2);
    s.insert(5);
    s.insert(9);

    // O(logN)
    s.erase(5);

    // begin
    cout << *s.begin() << endl;

    auto it = s.begin();
    printf("%p\n", it);
    cout << *it << endl;
    cout << *next(it) << endl;
    cout << *next(next(it)) << endl;
    line();

    it++;
    cout << *it << endl;
    line();


    // cout all
    for (auto itr = s.begin(); itr != s.end(); ++itr) {
        cout << *itr << endl;
    }
    line();


    // rbegin
    auto it2 = s.rbegin();
    printf("%p\n", it2);
    cout << *it2 << endl;
    line();


    // end
    auto it3 = s.end();
    cout << *prev(it3) << endl;
    line();


    // erase even num by iterator O(1)
    for (auto itr = s.begin(); itr != s.end();) {
        if (*itr % 2 == 0) {
            // return next iterator
            itr = s.erase(itr);
        } else {
            ++itr;
        }
    }
    cout_all(s);
    line();


    // erase 8 <= x <= 10
    auto low = s.lower_bound(8);
    auto high = s.upper_bound(10);
    auto it4 = low;
    while (it4 != high) {
        cout << *it4 << endl;
        ++it4;
    }
    line();

    s.erase(low, high);

    cout_all(s);
    line();


    // x not in set -> return end() iterator
    auto it5 = s.find(13);
    // return 0 or 1
    cout << s.count(13) << endl;
    // size
    if (!s.empty()) {
        cout << s.size() << endl;
    }

    return 0;
}


void line() {
    cout << "------" << endl;
}

void cout_all(set<int> s) {
    for (auto x: s) {
        cout << x << endl;
    }
}