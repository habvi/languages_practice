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
    // prefer?
    s.emplace(12);

    // O(logN)
    s.erase(5);

    // begin
    cout << *s.begin() << endl;

    auto itr = s.begin();
    printf("%p\n", itr);
    cout << *itr << endl;
    cout << *next(itr) << endl;
    cout << *next(next(itr)) << endl;
    line();

    itr++;
    cout << *itr << endl;
    line();


    // cout all
    for (auto itr = s.begin(); itr != s.end(); ++itr) {
        cout << *itr << endl;
    }
    line();


    // rbegin
    auto itr2 = s.rbegin();
    printf("%p\n", itr2);
    cout << *itr2 << endl;
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
    itr = low;
    while (itr != high) {
        cout << *itr << endl;
        ++itr;
    }
    line();

    s.erase(low, high);
    cout_all(s);
    line();


    // O(logN) if not in set -> return end() iterator
    itr = s.find(13);
    // O(logN) return 0 or 1
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