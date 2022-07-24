#include <bits/stdc++.h>
using namespace std;
#define _GLIBCXX_DEBUG
#define rep(i, a, b) for (int i = a; i < b; ++i)

// one-dimension access
bool check(char *a, int n) {
	rep(i, 0, n) {
		rep(j, 0, n) {
			cout << a[i * n + j];
		}
		cout << endl;
	}
	return true;
}

int main(void) {
	int n;
	cin >> n;

    // two-dimension array
	char a[n][n];
	rep(i, 0, n) {
		rep(j, 0, n) {
			cin >> a[i][j];
		}
	}

	check(&a[0][0], n);

	return 0;
}

/*
4
abcd
efgh
ijkl
mnop
*/