//           C - Duodecim Ferra
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc185/tasks/abc185_c

// AC
// ----------------------------------------

#include <bits/stdc++.h>
using namespace std;

long long combi(int n, int r) {
    r = min(r, n-r);
    long long res = 1;
    for (int i = 1; i <= r; i++) {
        res *= (n - i + 1);
        res /= i;
    }
    return res;
}


int main() {
    int N; cin >> N;

    cout << combi(N-1, 11) << endl;

}